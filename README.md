# low_card
Tool to test loading influxdb_iox with high throughput, low cardinality data


# Usage:
Run [influxdb_iox](https://github.com/influxdata/influxdb_iox) like:

```shell
cargo run --release  -- run all-in-one  --max-http-request-size=1000000000
```

The run low_card like:

```shell
cd low_card
cargo run --release
```
