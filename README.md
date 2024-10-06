# GreptimeDB GPX File Loader
![ci](https://github.com/v0y4g3r/gpx-greptimedb-loader/actions/workflows/rust.yml/badge.svg) [![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

[GPX](https://en.wikipedia.org/wiki/GPS_Exchange_Format) is a
widely used file format for recording locations and tracks.

gpx-greptimedb-loader provides a simple utility to write tracks in
GPX files to [GreptimeDB](https://github.com/GreptimeTeam/greptimedb)
for further analysis and visualization.

## Build

1. Make sure Rust toolchain is properly installed.
2. Download code
    ```bash
    git clone https://github.com/v0y4g3r/gpx-greptimedb-loader.git  
    ```

3. Go to project root and build
    ```bash
    cd gpx-greptimedb-loader
    cargo build --release
    ```
   And the binary locates in `./target/release/gpx-greptimedb-loader`.

## Usage

> Please ensure that GreptimeDB instance is started.

```bash
./gpx-greptimedb-loader \
  --track-name <track name> \
  --input <GPX file path> \
  --db-endpoint=<GreptimeDB endpoint> \
  --table-name=<target table name>
```

And now you can play with GEO-GreptimeDB!

```sql
mysql> select h3_latlng_to_cell_string(latitude, longitude, 15) from `gpx` 
		where ts > '2024-10-02 18:53:00' 
		and ts <= '2024-10-02 18:54:00';
+--------------------------------------------------------------------------+
| h3_latlng_to_cell_string(gpx.latitude,gpx.longitude,Int64(15)) |
+--------------------------------------------------------------------------+
| 8f29124cb8652e4                                                          |
| 8f29124cb86e230                                                          |
| 8f29124cb84c38c                                                          |
| 8f29124cb84b290                                                          |
| 8f29124cbbae984                                                          |
| 8f29124cbb8c1ac                                                          |
| 8f29124cbb8b718                                                          |
| 8f29124cbaaec24                                                          |
| 8f29124cba8ccd6                                                          |
| 8f29124cba8b418                                                          |
+--------------------------------------------------------------------------+
10 rows in set (0.04 sec)

mysql> select max(speed) from gpx;
+--------------------+
| MAX(gpx.speed)     |
+--------------------+
| 171.90617142857144 |
+--------------------+
1 row in set (0.05 sec)

mysql> select avg(speed) from gpx;
+-------------------+
| AVG(gpx.speed)    |
+-------------------+
| 60.90703674459004 |
+-------------------+
1 row in set (0.03 sec)
```

Guess what track is this?

## TODO
- [ ] Support encoding tracks to [GeoJSON line strings](https://datatracker.ietf.org/doc/html/rfc7946#section-3.1.4).
- [ ] Calculations while parsing GPX files.
  - [x] Speed
  - [ ] Cumulative distance
  - [ ] Slope
