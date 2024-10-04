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

//! Command line arguments.

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Unique name for the track to input.
    #[arg(short, long)]
    pub track_name: String,
    /// The path of GPX file to load.
    #[arg(short, long)]
    pub input: String,
    /// The endpoint of target GreptimeDB's gRPC service.
    #[arg(short = 'o', long, default_value = "localhost:4001")]
    pub db_endpoint: String,
    /// Username for specified GreptimeDB instance.
    #[arg(long)]
    pub db_username: Option<String>,
    /// Password for specified GreptimeDB instance.
    #[arg(long)]
    pub db_password: Option<String>,
    /// Table name to insert.
    #[arg(long, default_value = "gpx")]
    pub table_name: String,
    /// Target database name
    #[arg(long, default_value = "public")]
    pub database_name: String,
    /// whether to use TLS.
    #[arg(long, default_value_t = false)]
    pub use_tls: bool,
}
