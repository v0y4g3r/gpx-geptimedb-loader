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

//! Error variants

use gpx::errors::GpxError;
use snafu::{Location, Snafu};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Timestamp not present"))]
    TimestampNotPresent {
        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(display("Timestamp not present"))]
    WriteGreptimeDB {
        #[snafu(source)]
        error: greptimedb_ingester::Error,
        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(display("Cannot open GPX file, path: {}", path))]
    OpenGpxFile {
        path: String,
        #[snafu(source)]
        source: std::io::Error,
        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(display("Failed to read GPX file, path: {}", path))]
    ReadGpxFile {
        path: String,
        #[snafu(source)]
        source: GpxError,
        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(display(
        "Failed to calculate distance between {:?} and {:?}, source: {}",
        prev,
        next,
        msg
    ))]
    CalculateDistance {
        prev: geoutils::Location,
        next: geoutils::Location,
        msg: String,
    },
}
