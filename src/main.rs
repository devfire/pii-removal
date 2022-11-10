use clap::Parser;
use crate::arguments::Args;

mod arguments;

fn main() {
   let arguments = Args::parse();

   let files = arguments.files;
   let pattern = arguments.pattern;

   println!("Looking for pattern {} in {}", pattern, files);
}