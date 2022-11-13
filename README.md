# Overview
This solution removes PII from gzipped log files. 

What constitutes "PII" is passed as a `-p` parameter and must be a valid regex.

To run, `git clone` this repo and then execute `cargo build --release` to build an optimized binary.
# Requirements
- [x] accept as input one or more gzipped log files
- [x] for each input file, produce a redacted copy
- [x] create an audit log that includes the name of each file processed, a count of the total number of lines processed in each log file, and a count of the total number of lines removed from each log file
- [x] ​​must​​ not​​ alter ​​logs ​​in-place
- [x] ​​must ​​remove​ ​all ​​log​​lines ​​containing ​​sensitive ​​data​

# Sample usage
```console
igor@laptop:~/pii-removal$ cargo run --release -- -l out.log -p "SSN" test-data/*.gz  

2022-11-11T12:56:14.255265458-05:00 INFO pii_removal - Processing file: "test-data/bad.data.log.gz" 
2022-11-11T12:56:14.255331467-05:00 ERROR pii_removal - Encountered invalid gzip file, error: invalid gzip header
2022-11-11T12:56:14.255349507-05:00 INFO pii_removal - Lines processed: 1 Lines redacted: 0
2022-11-11T12:56:14.255362276-05:00 INFO pii_removal - Processing file: "test-data/sample.log.1.gz" 
2022-11-11T12:56:14.255540031-05:00 INFO pii_removal - Lines processed: 97 Lines redacted: 11
2022-11-11T12:56:14.255615100-05:00 INFO pii_removal - Processing file: "test-data/sample.log.2.gz" 
2022-11-11T12:56:14.255746364-05:00 INFO pii_removal - Lines processed: 97 Lines redacted: 11
```

# Notes
- One of the gzipped files is intentionally bad: `bad.data.log.gz` is an invalid gzip file to test the error handling.
- Log file is intentionally preserved between runs to allow for comparison
- All other files are re-created idempotently
