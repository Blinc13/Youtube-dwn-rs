use clap::Parser;

#[derive(Parser)]
#[clap(about, author, version)]
pub struct Args {
    pub url: String,

    #[clap(short, long)]
    pub format: String,

    #[clap(long)]
    /// Prints every possible format for video
    pub show_formats: bool
}