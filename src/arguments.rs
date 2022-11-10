use clap::Parser;

// NOTE: we need multiple values since shell glob expansion will create N number of files.
// https://docs.rs/clap/latest/clap/builder/struct.Arg.html#method.get_value_delimiter
#[derive(Parser,Default,Debug)]
pub struct Args {
    /// Files glob pattern to process
    #[arg(short = 'f', use_value_delimiter = true, value_delimiter = ' ', long)] //
    pub files: Option<Vec<String>>,

    // Pipe separated regex PII pattern to eliminate, i.e. "CC|SSN"
    //#[clap(short = 'p', long)]
    //pub pattern: String,
}
