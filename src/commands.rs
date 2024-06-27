use std::usize;

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
    #[arg(long, default_value_t = true)] // todo change default value once other branch is built
    pub pretty: bool,

    #[arg(long, default_value = ",")]
    pub delimiter: char,

    #[arg(short = 'c', long, value_delimiter = ',')]
    pub columns: Vec<String>,

    // todo implementing ranges will come later
    // for now we stick with simple start - end arguments
    // #[arg(short = 'r', long, value_delimiter = ',')]
    // pub rows: Vec<u32>,
    //
    #[arg(short, long, default_value_t = 0)]
    pub start: usize,

    #[arg(short, long, default_value_t = usize::MAX)]
    pub end: usize,

    #[arg(short = 'f', long)]
    pub filter: Option<String>,

    #[arg(long, alias = "sr")]
    pub show_row_nums: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Show(ShowArgs),
}

#[derive(Debug, Args)]
pub struct ShowArgs {
    #[arg()]
    pub file_path: String,

    #[arg(long)]
    pub head: bool,

    #[arg(long)]
    pub tail: bool,
}
