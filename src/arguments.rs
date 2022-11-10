use clap::Parser;

// NOTE: we need multiple values since shell glob expansion will create N number of files.
// https://docs.rs/clap/latest/clap/builder/struct.Arg.html#method.get_value_delimiter
#[derive(Debug, Parser)]
#[clap(
    version,
    about = "Remove sensitive PII from log files."
)]
pub struct Args {
    /// Files glob pattern to process
    #[clap(short, long, use_value_delimiter = true)]
    pub files: Option<Vec<String>>,
}
