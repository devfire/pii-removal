# Overview
This solution removes PII from gzipped log files.

To run, `git clone` this repo and then execute `cargo run -- test-data/*.gz`

NOTE: one of the gzipped files is intentionally bad: `bad.data.log.gz` is an invalid gzip file to test the error handling.
