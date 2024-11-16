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
    #[arg(long, default_value = ",")]
    pub delimiter: char,

    /// needed for sorting. otherwise strings will be compared and not the actual types
    /// works for ints and floats
    #[arg(long)]
    pub infer_types: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Show(ShowArgs),
}

#[derive(Debug, Args)]
pub struct ShowArgs {
    #[arg()]
    pub file_path: String,

    // todo change to format instead of pretty then allow several options via enums
    #[arg(long, default_value_t = true)] // todo change default value once other branch is built
    pub pretty: bool,

    #[arg(long)]
    pub head: bool,

    #[arg(long)]
    pub tail: bool,

    // todo implementing ranges will come later
    // for now we stick with simple start - end arguments
    // #[arg(short = 'r', long, value_delimiter = ',')]
    // pub rows: Vec<u32>,
    /// set beginning row which should be shown, sorting does not affect this row position
    #[arg(short, long, default_value_t = 0)]
    pub start: usize,

    /// set end row which should be shown, sorting does not affect this row position
    #[arg(short, long, default_value_t = usize::MAX)]
    pub end: usize,

    /// The column key on which the table should be sorted. Required if sort is true.
    #[arg(long)]
    pub sort_key: Option<String>,

    /// the format according to which a datetime column should be sorted.
    #[arg(long, default_value = "")]
    pub dformat: String,

    /// Which columns should be shown, reorders the columns according to input.
    /// Several columns can be provided. Use either col_index as int or the the name
    #[arg(short = 'c', long, value_delimiter = ',')]
    pub columns: Vec<String>,

    /// a regex filter applied.
    /// per default applied to all columns. Set filter-cols to limit matching to certain columns
    #[arg(short = 'f', long)]
    pub filter: Option<String>,

    /// Which columns should be shown, reorders the columns according to input.
    /// Several columns can be provided. Use either col_index as int or the the name
    #[arg(alias = "fc", long, value_delimiter = ',')]
    pub filter_cols: Vec<String>,

    /// add the row number as index to the output
    #[arg(long, alias = "sr")]
    pub show_row_nums: bool,

    #[arg(long, alias = "desc", default_value_t = false)]
    pub descending: bool,
}
