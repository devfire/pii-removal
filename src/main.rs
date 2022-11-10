/*
There's a way to write a regex! macro which compiles regular expressions when the program compiles. 
In other words, using regex! means our program cannot compile with an invalid regular expression. 
Moreover, the regex! macro compiles the given expression to native Rust code, which makes it much faster for searching text.

To use the regex! macro, we must enable the plugin feature and import the regex_macros crate as a syntax extension: 

Reference: https://docs.rs/regex/0.1.26/regex/

*/
#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

use clap::Parser;
use crate::arguments::Args;
use flate2::read::GzDecoder;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};


mod arguments;

fn main() -> io::Result<()> {
   let arguments = Args::parse();

   let files = arguments.files;
   let pattern = arguments.pattern;

   println!("Looking for pattern {} in {}", pattern, files);

   let file = File::open(files)?;
   
   let reader = BufReader::new(GzDecoder::new(file));

   let mut lines_processed = 0;
   let mut lines_redacted = 0;




   for line in reader.lines() {
       println!("{}", line?);
       lines_processed = lines_processed + 1;
       
   }

   reader
   .lines()
   .filter_map(|line| line.ok())
   .filter(|line| regex_set.is_match(line.as_str()))
   .for_each(|x| println!("{}", x));

   Ok(())
}