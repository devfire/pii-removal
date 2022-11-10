use clap::Parser;
use crate::arguments::Args;
use flate2::read::GzDecoder;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};


mod arguments;

fn main() -> io::Result<()> {
   let arguments = Args::parse();

   let files = arguments.files;
   let pattern = "CC|SSN"; // make sure to return a &str here

   println!("Looking for pattern {} in {}", pattern, files);

   let file = File::open(files)?;
   
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

   Ok(())
}