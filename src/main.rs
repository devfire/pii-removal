use clap::Parser;
//use crate::arguments::Args;
use flate2::read::GzDecoder;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

// NOTE: we need multiple values since shell glob expansion will create N number of files.
// https://docs.rs/clap/latest/clap/builder/struct.Arg.html#method.get_value_delimiter
#[derive(Debug, Parser)]
#[clap(
    version,
    about = "Remove sensitive PII from log files."
)]
struct Args {
    /// Files glob pattern to process
    #[clap(short, long, use_value_delimiter = true)]
    pub files: Option<Vec<String>>,
}


//mod arguments;

fn main() -> io::Result<()> {
   let arguments = Args::parse();

   let pattern = "CC|SSN"; // make sure to return a &str here

   // This took a long time to get right.
   // What we get back from arguments.files is an Option of a Vector of files.
   // So we are using a shorthand to grab the vector from the option,
   // iterating over the values in the vector.
   if let Some(files) = arguments.files {
        for file in files {
            println!("Processing {}", file);
            let file = File::open(file)?;
       
        let reader = BufReader::new(GzDecoder::new(file));
     
        let mut lines_processed = 0;
        let mut lines_redacted = 0;
     
        let re = match Regex::new(pattern) {
             Ok(re) => re,
             Err(err) => panic!("{}", err),
         };
        
        for read_line_result in reader.lines() {
            //println!("{}", line?);
            lines_processed = lines_processed + 1;
     
            match read_line_result {
                 Ok(read_line) => {
                     if re.is_match(&read_line) {
                         println!("{}", read_line);
                         lines_redacted = lines_redacted + 1;
                     }
                 },
                 Err(e) => return Err(e),
            };
     
        }


        println!("Lines processed: {} Lines redacted: {}", lines_processed, lines_redacted);
        }
   }

   Ok(())
}