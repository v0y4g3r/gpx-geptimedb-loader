# GreptimeDB GPX File Loader

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
mysql> select h3_latlng_to_cell(latitude, longitude, 15) from `gpx` 
            where ts > '2024-10-02 18:53:00' 
              and ts <= '2024-10-02 18:54:00';
+-------------------------------------------------------------------+
| h3_latlng_to_cell(gpx.latitude,gpx.longitude,Int64(15)) |
+-------------------------------------------------------------------+
|                                                644737283886830308 |
|                                                644737283886866992 |
|                                                644737283886728076 |
|                                                644737283886723728 |
|                                                644737283890276740 |
|                                                644737283890135468 |
|                                                644737283890132760 |
|                                                644737283889228836 |
|                                                644737283889089750 |
|                                                644737283889083416 |
+-------------------------------------------------------------------+
10 rows in set (0.02 sec)
```

## TODO
- [ ] Support encoding tracks to [GeoJSON line strings](https://datatracker.ietf.org/doc/html/rfc7946#section-3.1.4).
- [ ] Calculate speed/slope while parsing GPX files.


