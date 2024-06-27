use csv::ReaderBuilder;
use prettytable::{
    color,
    format::{self},
    Attr, Cell, Row, Table,
};

use crate::commands;

pub fn parse_and_display_csv(
    common: &commands::CommonArgs,
    args: &commands::ShowArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create CSV reader
    let mut rdr = ReaderBuilder::new().from_path(args.file_path.clone())?;

    // Get the headers
    let headers = rdr.headers()?.clone();

    // Determine indices of columns to display
    let indices: Vec<usize> = if common.columns.is_empty() {
        (0..headers.len()).collect()
    } else {
        common
            .columns
            .iter()
            .map(|col| {
                headers
                    .iter()
                    .position(|h| h == col)
                    .or_else(|| col.parse::<usize>().ok().filter(|&i| i < headers.len()))
                    .expect("Invalid column name or index")
            })
            .collect()
    };

    if common.pretty {
        // Create a table and add headers
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        let header_row: Vec<Cell> = indices
            .iter()
            .map(|&i| {
                Cell::new(&headers[i])
                    .with_style(Attr::Bold)
                    .with_style(Attr::ForegroundColor(color::CYAN))
            })
            .collect();
        table.set_titles(Row::new(header_row));

        // Add CSV records to the table
        for result in rdr.records() {
            let record = result?;
            let row: Vec<Cell> = indices.iter().map(|&i| Cell::new(&record[i])).collect();
            table.add_row(Row::new(row));
        }

        // Print the table
        table.printstd();
    } else {
    }

    Ok(())
}
