use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "rscsv")]
#[command(about = "Work and handle csv files in the shell", long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub common: CommonArgs,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Args)]
pub struct CommonArgs {
    // todo change to format instead of pretty then allow several options via enums
    #[arg(long)]
    pub pretty: bool,

    #[arg(long, default_value = ",")]
    pub delimiter: char,

    #[arg(short = 'c', long, value_delimiter = ',')]
    pub columns: Vec<String>,

    #[arg(short = 'r', long, value_delimiter = ',')]
    pub rows: Vec<u32>,

    #[arg(short = 'f', long)]
    pub filter: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Show(ShowArgs),
}

#[derive(Debug, Args)]
pub struct ShowArgs {
    #[arg()]
    pub file_path: String,

    #[arg(short, long)]
    pub head: bool,

    #[arg(short, long)]
    pub last: bool,
}
