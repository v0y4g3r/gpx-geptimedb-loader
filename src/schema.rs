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

use greptimedb_ingester::api::v1::{ColumnDataType, ColumnSchema, SemanticType};

/// Creates schema for GPX data.
pub fn gpx_schema() -> Vec<ColumnSchema> {
    let mut fields = Vec::with_capacity(15);
    fields.push(ColumnSchema {
        column_name: "name".to_string(),
        datatype: ColumnDataType::String as i32,
        semantic_type: SemanticType::Tag as i32,
        datatype_extension: None,
    });

    fields.push(ColumnSchema {
        column_name: "track".to_string(),
        datatype: ColumnDataType::Uint32 as i32,
        semantic_type: SemanticType::Tag as i32,
        datatype_extension: None,
    });

    fields.push(ColumnSchema {
        column_name: "segment".to_string(),
        datatype: ColumnDataType::Uint32 as i32,
        semantic_type: SemanticType::Tag as i32,
        datatype_extension: None,
    });

    fields.push(ColumnSchema {
        column_name: "ts".to_string(),
        datatype: ColumnDataType::TimestampSecond as i32,
        semantic_type: SemanticType::Timestamp as i32,
        datatype_extension: None,
    });

    fields.extend(
        [
            ("latitude", ColumnDataType::Float64),
            ("longitude", ColumnDataType::Float64),
            ("elevation", ColumnDataType::Float64),
            ("geoidheight", ColumnDataType::Float64),
            ("hdop", ColumnDataType::Float64),
            ("vdop", ColumnDataType::Float64),
            ("pdop", ColumnDataType::Float64),
            ("comment", ColumnDataType::String),
            ("description", ColumnDataType::String),
            ("source", ColumnDataType::String),
            ("symbol", ColumnDataType::String),
            ("sat", ColumnDataType::Uint64),
        ]
        .into_iter()
        .map(|(name, ty)| ColumnSchema {
            column_name: name.to_string(),
            datatype: ty as i32,
            semantic_type: SemanticType::Field as i32,
            datatype_extension: None,
        }),
    );

    fields
}
