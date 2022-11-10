use clap::Parser;

#[derive(Parser,Default,Debug)]
pub struct Args {
    /// Files glob pattern to process
    #[clap(short = 'f', long)]
    pub files: String,

    /// Comma separated PII pattern to eliminate, i.e. "CC,SSN"
    #[clap(short = 'p', long)]
    pub pattern: String,
}
