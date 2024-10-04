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
./target/debug/gpx-greptimedb-loader \
  --track-name <track name> \
  --input <GPX file path> \
  --db-endpoint=<GreptimeDB endpoint> \
  --table-name=<target table name>
```


