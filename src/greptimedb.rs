// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::args::Args;
use crate::{error, schema};
use gpx::Waypoint;
use greptimedb_ingester::api::v1::auth_header::AuthScheme;
use greptimedb_ingester::api::v1::value::ValueData;
use greptimedb_ingester::api::v1::{Basic, Row, RowInsertRequest, RowInsertRequests, Rows, Value};
use greptimedb_ingester::{
    ChannelConfig, ChannelManager, ClientBuilder, ClientTlsOption, Database,
};
use snafu::{OptionExt, ResultExt};
use time::OffsetDateTime;

/// GreptimeDB client.
pub struct Client {
    /// Target table name.
    table_name: String,
    /// GreptimeDB instance.
    database: Database,
}

impl Client {
    /// Creates a new GreptimeDB instance.
    pub fn new(args: &Args) -> error::Result<Client> {
        let greptimedb_endpoint = args.db_endpoint.clone();
        let greptimedb_dbname = args.database_name.clone();

        let builder = ClientBuilder::default()
            .peers(vec![&greptimedb_endpoint])
            .compression(greptimedb_ingester::Compression::Gzip);

        let grpc_client = if args.use_tls {
            let channel_manager = ChannelManager::with_tls_config(
                ChannelConfig::default().client_tls_config(ClientTlsOption::default()),
            )
            .expect("Failed to create channel manager");
            builder.channel_manager(channel_manager).build()
        } else {
            builder.build()
        };

        let mut client = Database::new_with_dbname(greptimedb_dbname, grpc_client);

        match (&args.db_username, &args.db_password) {
            (Some(user_name), Some(password)) => client.set_auth(AuthScheme::Basic(Basic {
                username: user_name.clone(),
                password: password.clone(),
            })),
            (None, None) => {}
            _ => {
                panic!("Database username and password must be both present or not")
            }
        };
        Ok(Self {
            table_name: args.table_name.clone(),
            database: client,
        })
    }

    /// Writes a set of [Waypoint]s.
    pub async fn write(
        &self,
        track_name: &str,
        track_id: u32,
        segment_id: u32,
        waypoints: Vec<Waypoint>,
    ) -> error::Result<u32> {
        let request = to_insert_request(
            &self.table_name,
            track_name,
            track_id,
            segment_id,
            waypoints,
        )?;
        self.database
            .row_insert(request)
            .await
            .context(error::WriteGreptimeDBSnafu)
    }
}

/// Converts [Waypoint]s to [RowInsertRequests].
pub fn to_insert_request(
    table_name: &str,
    name: &str,
    track: u32,
    segment: u32,
    records: Vec<Waypoint>,
) -> error::Result<RowInsertRequests> {
    let rows = records
        .into_iter()
        .map(|p| waypoint_to_row(name, track, segment, p))
        .collect::<error::Result<_>>()?;
    Ok(RowInsertRequests {
        inserts: vec![RowInsertRequest {
            table_name: table_name.to_string(),
            rows: Some(Rows {
                schema: schema::gpx_schema(),
                rows,
            }),
        }],
    })
}

/// Converts a [Waypoint] struct to a [Row] in GreptimeDB.
fn waypoint_to_row(name: &str, track: u32, segment: u32, p: Waypoint) -> error::Result<Row> {
    let time = p
        .time
        .as_ref()
        .map(|t| OffsetDateTime::from(*t))
        .context(error::TimestampNotPresentSnafu)?;
    let ts = time.unix_timestamp();
    Ok(Row {
        values: vec![
            Value {
                value_data: Some(ValueData::StringValue(name.to_string())),
            },
            Value {
                value_data: Some(ValueData::U32Value(track)),
            },
            Value {
                value_data: Some(ValueData::U32Value(segment)),
            },
            Value {
                value_data: Some(ValueData::TimestampSecondValue(ts)),
            },
            Value {
                value_data: Some(ValueData::F64Value(p.point().y())),
            },
            Value {
                value_data: Some(ValueData::F64Value(p.point().x())),
            },
            Value {
                value_data: p.elevation.map(ValueData::F64Value),
            },
            Value {
                value_data: p.geoidheight.map(ValueData::F64Value),
            },
            Value {
                value_data: p.hdop.map(ValueData::F64Value),
            },
            Value {
                value_data: p.vdop.map(ValueData::F64Value),
            },
            Value {
                value_data: p.pdop.map(ValueData::F64Value),
            },
            Value {
                value_data: p
                    .comment
                    .map(ValueData::StringValue),
            },
            Value {
                value_data: p
                    .description
                    .map(ValueData::StringValue),
            },
            Value {
                value_data: p.source.map(ValueData::StringValue),
            },
            Value {
                value_data: p.symbol.map(ValueData::StringValue),
            },
            Value {
                value_data: p.sat.map(ValueData::U64Value),
            },
        ],
    })
}

#[cfg(test)]
mod tests {
    use crate::greptimedb::waypoint_to_row;
    use std::io::BufReader;

    #[test]
    fn test_convert() {
        let reader = BufReader::new(
            std::fs::OpenOptions::new()
                .read(true)
                .open("tests/assets/wikipedia_example.gpx")
                .unwrap(),
        );
        let gpx = gpx::read(reader).unwrap();
        let waypoint = &gpx.tracks[0].segments[0].points[0];
        let row = waypoint_to_row("test", 0, 0, waypoint.clone()).unwrap();
        assert_eq!(16, row.values.len());
    }
}
