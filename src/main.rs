use clap::Parser;
use flate2::read::GzDecoder;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write, BufRead, BufReader};
use log::{error, info};

mod logger;

// NOTE the arg_required_else_help parameter. It forces a default help when no CLI inputs are passed.
// It is undocumented but does exist, see here
// https://github.com/clap-rs/clap/blob/master/examples/git-derive.rs#L19
#[derive(Parser)]
#[command(author, version, about, arg_required_else_help = true, long_about = None)]
struct Cli {
    ///List of files to process, wildcards are supported.
    files: Vec<String>,
}

// PII patterns to filter. 
// TODO: Should be configurable from the CLI.
const PATTERN: &str = "CC|SSN"; // make sure to return a &str here


// this gets appended to the end of the redacted file
const REDACTED_SUFFIX: &str = ".redacted";

fn main() -> io::Result<()> {
    // Parse the arguments coming in from the CLI
    let cli = Cli::parse();

    // Setup the logging framework
    if let Err(e) = logger::init() {
        error!("Could not initialize logger: {}", e);
    }

    let re = match Regex::new(PATTERN) {
        Ok(re) => re,
        Err(err) => panic!("{}", err),
    };

    // cli.files is a Vector of strings, containing 1 or more files to process
    for file in cli.files {
        info!("Processing file: {:?} ", file);

        // constuct a new filename for the target output file with PII removed
        // https://doc.rust-lang.org/std/macro.format.html
        // NOTE: this will create a "redacted" output file even if the input is not a valid gzip
        // TODO: run a quick gzip header validation to ensure a valid gzip input
        let redacted_file_name = format!("{}{}",file, REDACTED_SUFFIX);

        // Open the gz input file read-only
        let input_file = File::open(file)?;

        // Create the file without PII write-only
        let mut redacted_file = BufWriter::new(File::create(redacted_file_name)?);

        // NOTE: the GzDecoder is already a buffered implementation. 
        // However, it has no idea about any line breaks.
        // Second BufReader is to identify the line breaks and return whole lines.
        let reader = BufReader::new(GzDecoder::new(input_file));
     
        // Total number of lines processed, per file
        let mut lines_processed = 0;

        // Total number of PII lines removed, per file
        let mut lines_redacted = 0;

        // Stream the gzip file contents, one line at a time
        for read_line_result in reader.lines() {
            
            // Bump up the counter
            lines_processed = lines_processed + 1;
     
            // read_line_result is a Result enum, so we will match on both
            // OK() and Err() arms.
            match read_line_result {
                 Ok(read_line) => {
                     if re.is_match(&read_line) {
                         lines_redacted = lines_redacted + 1;
                     } else {
                        writeln!(redacted_file, "{}", read_line)?;
                     }
                 },
                 Err(e) => error!("Encountered invalid gzip file, error: {}", e)
            };
        }
        info!("Lines processed: {} Lines redacted: {}", lines_processed, lines_redacted);
    }
    Ok(())
}