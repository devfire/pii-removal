use clap::Parser;
use crate::arguments::Args;
use flate2::read::GzDecoder;
//use std::fmt;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};

/// Iterator of lines direct from file path.
pub fn read_lines(path: &str) -> io::Result<Lines<BufReader<File>>> {
    Ok(BufReader::new(File::open(path)?).lines())
}

/// Iterator of lines, gunzipped, direct from file path.
pub fn read_gz_lines(path: &str) -> io::Result<Lines<BufReader<GzDecoder<File>>>> {
    Ok(BufReader::new(GzDecoder::new(File::open(path)?)).lines())
}

mod arguments;

fn main() -> io::Result<()> {
   let arguments = Args::parse();

   let files = arguments.files;
   let pattern = arguments.pattern;

   println!("Looking for pattern {} in {}", pattern, files);

   let file = File::open(files)?;
   
   let reader = BufReader::new(GzDecoder::new(file));

   for line in reader.lines() {
       println!("{}", line?);
   }

   Ok(())
}