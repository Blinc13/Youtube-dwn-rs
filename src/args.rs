use clap::*;

#[derive(Parser)]
#[clap(about, author, version)]
pub struct Args {
    pub url: String,

    #[command(subcommand)]
    pub command: Command
}

#[derive(Subcommand)]
pub enum Command {
    Download {
        format: String,
        workers_count: Option<usize>
    },
    Meta
}