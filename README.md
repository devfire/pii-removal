# Overview
This solution removes PII from gzipped log files.

To run, `git clone` this repo and then execute `cargo build --release` to build an optimized binary.

# Sample usage
```bash
target/release/pii-removal test-data/*.gz --logfile log.out
```

# Notes
One of the gzipped files is intentionally bad: `bad.data.log.gz` is an invalid gzip file to test the error handling.
