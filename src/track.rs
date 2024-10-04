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

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;

    #[tokio::test]
    async fn test_read_gpx() {
        let reader = BufReader::new(File::open("/home/lei/1_cst-strlt.gpx").unwrap());
        let gpx = gpx::read(reader).unwrap();
        gpx.tracks[0].segments[0].points[0].time.unwrap();
    }
}
