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
use crate::error::OpenGpxFileSnafu;
use crate::greptimedb::Client;
use clap::Parser;
use snafu::ResultExt;
use std::fs::File;
use std::io::BufReader;

mod args;
mod error;
mod greptimedb;
mod schema;
mod track;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let db = Client::new(&args).unwrap();

    let reader = BufReader::new(
        File::open(&args.input)
            .context(OpenGpxFileSnafu { path: &args.input })
            .unwrap(),
    );
    let gpx = gpx::read(reader)
        .context(error::ReadGpxFileSnafu { path: &args.input })
        .unwrap();

    for (track_id, track) in gpx.tracks.into_iter().enumerate() {
        for (seg_id, seg) in track.segments.into_iter().enumerate() {
            db.write(&args.track_name, track_id as u32, seg_id as u32, seg.points)
                .await
                .unwrap();
        }
    }
}
