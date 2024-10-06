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

use crate::error;
use geoutils::Location;
use gpx::{TrackSegment, Waypoint};
use snafu::OptionExt;
use time::OffsetDateTime;

/// Fills the missing speed (km/h) in [Waypoint] in segments.
pub fn fill_speed_on_missing(segment: &mut TrackSegment) -> error::Result<()> {
    let num_points = segment.points.len();
    if num_points == 0 {
        return Ok(());
    }

    let mut prev_idx = 0;
    for idx in 1..num_points {
        let prev = &segment.points[prev_idx];
        let p = &segment.points[idx];

        if p.speed.is_none() {
            let distance = calculate_distance(p, prev)?;
            let duration = OffsetDateTime::from(p.time.context(error::TimestampNotPresentSnafu)?)
                - OffsetDateTime::from(prev.time.context(error::TimestampNotPresentSnafu)?);
            let speed = distance / duration.as_seconds_f64() * 3.6;
            segment.points[idx].speed = Some(speed);
        }
        prev_idx = idx;
    }
    if num_points > 1 {
        segment.points[0].speed = segment.points[1].speed;
    }
    Ok(())
}

/// Calculates distance in meters between two [Waypoint]s using Vincenty's formulae.
fn calculate_distance(prev: &Waypoint, next: &Waypoint) -> error::Result<f64> {
    let prev = Location::new(prev.point().y(), prev.point().x());
    let next = Location::new(next.point().y(), next.point().x());
    let distance = next
        .distance_to(&prev)
        .map_err(|e| error::Error::CalculateDistance { prev, next, msg: e })?;
    Ok(distance.meters())
}

#[cfg(test)]
mod tests {
    use crate::track::{calculate_distance, fill_speed_on_missing};
    use approx::relative_eq;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn test_read_gpx() {
        let reader = BufReader::new(File::open("tests/assets/wikipedia_example.gpx").unwrap());
        let gpx = gpx::read(reader).unwrap();
        gpx.tracks[0].segments[0].points[0].time.unwrap();
    }

    #[test]
    fn test_calculate_distance() {
        let reader = BufReader::new(File::open("tests/assets/wikipedia_example.gpx").unwrap());
        let gpx = gpx::read(reader).unwrap();
        let points = &gpx.tracks[0].segments[0].points;

        let prev = &points[0];
        let next = &points[2];
        let distance = calculate_distance(prev, next).unwrap();
        assert!(relative_eq!(430.0, distance, epsilon = 1.0))
    }

    #[test]
    fn test_fill_speed_on_missing() {
        let reader = BufReader::new(File::open("tests/assets/wikipedia_example.gpx").unwrap());
        let mut gpx = gpx::read(reader).unwrap();
        let segment = &mut gpx.tracks[0].segments[0];
        fill_speed_on_missing(segment).unwrap();
        assert!(segment.points.iter().all(|p| { p.speed.is_some() }));
    }

    #[test]
    fn test_fill_speed_in_empty_segment() {
        let reader = BufReader::new(File::open("tests/assets/wikipedia_example.gpx").unwrap());
        let mut gpx = gpx::read(reader).unwrap();
        let segment = &mut gpx.tracks[0].segments[0];

        let mut seg1 = segment.clone();
        seg1.points.clear();
        fill_speed_on_missing(&mut seg1).unwrap();
        assert!(seg1.points.iter().all(|p| { p.speed.is_some() }));

        let mut seg2 = segment.clone();
        seg2.points.drain(1..seg2.points.len());
        assert_eq!(1, seg2.points.len());
        fill_speed_on_missing(&mut seg2).unwrap();
        assert!(seg2.points[0].speed.is_none());

        let mut seg3 = segment.clone();
        seg3.points.drain(2..seg3.points.len());
        fill_speed_on_missing(&mut seg3).unwrap();
        assert!(seg3.points.iter().all(|p| { p.speed.is_some() }));
    }
}
