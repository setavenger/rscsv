use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use regex::Regex;
use std::cmp::Ordering;
use std::error::Error;

use csv::{ReaderBuilder, StringRecord};
use prettytable::{
    color,
    format::{self},
    Attr, Cell, Row, Table,
};

use crate::commands;

#[derive(Debug)]
pub enum DataTypes {
    String,
    Float,
    Integer,
    Datetime,
}

fn detect_column_type(records: &[StringRecord], column_index: usize) -> DataTypes {
    let mut is_integer = true;
    let mut is_float = true;

    // todo check entire column for consistency with regards to type and say where it fails to do a
    // column consistent conversion
    for record in records {
        if let Some(value) = record.get(column_index) {
            if value.is_empty() {
                continue;
            }
            // Check for integer
            if value.parse::<i64>().is_err() {
                is_integer = false;
            }
            // Check for float if it's not an integer
            if value.parse::<f64>().is_err() {
                is_float = false;
            }
            // If it's neither integer nor float, no need to continue checking
            if !is_integer && !is_float {
                break;
            }
        } else {
            // In case of parsing errors, default to string
            is_integer = false;
            is_float = false;
            break;
        }
    }

    if is_integer {
        DataTypes::Integer
    } else if is_float {
        DataTypes::Float
    } else {
        DataTypes::String
    }
}

// Sorting function based on type
fn sort_records(
    records: &mut [StringRecord],
    column_index: usize,
    column_type: DataTypes,
    datetime_format: &str,
) {
    match column_type {
        DataTypes::Integer => {
            records.sort_by(|a, b| {
                a[column_index]
                    .parse::<i64>()
                    .unwrap_or_default()
                    .cmp(&b[column_index].parse::<i64>().unwrap_or_default())
            });
        }
        DataTypes::Float => {
            records.sort_by(|a, b| {
                a[column_index]
                    .parse::<f64>()
                    .unwrap_or_default()
                    .partial_cmp(&b[column_index].parse::<f64>().unwrap_or_default())
                    .unwrap_or(Ordering::Equal)
            });
        }
        DataTypes::Datetime => {
            records.sort_by(|a, b| {
                parse_datetime(&a[column_index], datetime_format)
                    .unwrap()
                    .partial_cmp(&parse_datetime(&b[column_index], datetime_format).unwrap())
                    .unwrap_or(Ordering::Equal)
            });
        }
        _ => {
            // Default to string comparison
            records.sort_by(|a, b| a[column_index].cmp(&b[column_index]));
        }
    }
}

fn filter_records(records: &mut Vec<StringRecord>, pattern: &str, columns: Option<Vec<usize>>) {
    let re = Regex::new(pattern).expect("Invalid regex pattern");
    records.retain(|record| match &columns {
        Some(cols) => cols.iter().any(|&col_index| {
            record
                .get(col_index)
                .map_or(false, |field| re.is_match(field))
        }),
        None => record.iter().any(|field| re.is_match(field)),
    })
}

pub fn parse_and_display_csv(
    common: &commands::CommonArgs,
    args: &commands::ShowArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create CSV reader
    let mut rdr = ReaderBuilder::new()
        .delimiter(common.delimiter as u8)
        .from_path(args.file_path.clone())?;

    // Get the headers
    let mut headers = rdr.headers()?.clone();

    // Determine indices of columns to display
    let indices: Vec<usize> = if args.columns.is_empty() {
        (0..headers.len()).collect()
    } else {
        args.columns
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

    // todo: merge the two conditionals?
    if args.show_row_nums {
        let mut new_indices = Vec::with_capacity(indices.len() + 1);
        new_indices.push(0);
        new_indices.extend(indices.iter().map(|num| num + 1).collect::<Vec<usize>>());

        let mut new_headers = StringRecord::new();
        new_headers.push_field("index");
        new_headers.extend(headers.iter());
        headers = new_headers;
    }

    // Read records
    let records: Vec<StringRecord> = rdr.records().filter_map(Result::ok).collect();

    // Example sorting (by the first selected column, ascending)
    let mut sorted_records = records;
    // Adding index to each StringRecord
    // Iterate over the records with indices
    if args.show_row_nums {
        for (index, record) in sorted_records.iter_mut().enumerate() {
            let mut new_record = StringRecord::new();
            new_record.push_field(&index.to_string()); // Prepend the index

            // Append existing fields from the record
            for field in record.iter() {
                new_record.push_field(field);
            }

            // Replace the old record with the new one
            *record = new_record;
        }
    }

    if let Some(pattern) = &args.filter {
        // let pattern = args.filter.as_ref().unwrap();
        let indices: Vec<usize> = if args.filter_cols.is_empty() {
            (0..headers.len()).collect()
        } else {
            args.filter_cols
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
        filter_records(&mut sorted_records, pattern, Some(indices))
    }

    if let Some(sort_key) = &args.sort_key {
        let col_index = headers
            .iter()
            .position(|h| h == sort_key)
            .or_else(|| {
                sort_key
                    .parse::<usize>()
                    .ok()
                    .filter(|&i| i < headers.len())
            })
            .expect("Invalid column name or index as sort_key");

        let mut column_type = detect_column_type(&sorted_records, col_index);
        if !args.dformat.is_empty() {
            column_type = DataTypes::Datetime;
        }
        sort_records(&mut sorted_records, col_index, column_type, &args.dformat);

        // simple way to achieve a descending order if we always just sort by ascending order
        if args.descending {
            sorted_records.reverse()
        }
    }

    if args.pretty {
        // Create a table and add formatting
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        let mut header_row: Vec<Cell> = Vec::new();

        // Append other headers
        header_row.extend(indices.iter().map(|&i| {
            Cell::new(&headers[i])
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::CYAN))
        }));
        table.set_titles(Row::new(header_row));

        // Add rows to the table
        for (idx, record) in sorted_records.iter().enumerate() {
            if idx < args.start {
                continue; // Skip rows before the start index
            }
            if idx > args.end {
                break; // Stop iterating once past the end index
            }

            let row = indices.iter().map(|&i| Cell::new(&record[i])).collect();
            table.add_row(Row::new(row));
        }

        // Print the table
        table.printstd();
    } else {
        todo!()
    }

    Ok(())
}

// outsource to todo

// Function to try parsing a string into various time formats
fn parse_datetime(input: &str, format: &str) -> Result<NaiveDateTime, Box<dyn Error>> {
    // Try parsing as a full datetime
    let datetime = NaiveDateTime::parse_from_str(input, format)
        .or_else(|_| {
            // If datetime fails, try parsing as just a date
            NaiveDate::parse_from_str(input, format).map(|d| d.and_hms_opt(0, 0, 0).unwrap())
        })
        .or_else(|_| {
            // If date fails, try parsing as just a time
            NaiveTime::parse_from_str(input, format).map(|t| {
                // Use Unix epoch start date with parsed time
                let unix_epoch_start = NaiveDate::default();
                NaiveDateTime::new(unix_epoch_start, t)
            })
        })?;

    Ok(datetime)
}

#[cfg(test)]
mod test {
    use chrono::{NaiveDate, NaiveDateTime};

    use crate::commands::{CommonArgs, ShowArgs};

    use super::parse_and_display_csv;

    #[test]
    fn simple_show() {
        let common_args = CommonArgs {
            delimiter: ',',
            infer_types: false,
        };

        let show_args = ShowArgs {
            file_path: "./test-data/test-simple.csv".to_string(),
            pretty: true,
            columns: vec![],
            start: 0,
            end: usize::MAX,
            filter: None,
            show_row_nums: true,
            head: false,
            tail: false,
            filter_cols: vec![],
            sort_key: Some("natural".to_string()),
            dformat: "".to_string(),
            descending: false,
        };
        let parse_and_display_csv = parse_and_display_csv(&common_args, &show_args);
        match parse_and_display_csv {
            Ok(_) => println!("was ok"),
            Err(e) => println!("err: {}", e),
        }
    }

    #[test]
    fn sorted_table() {
        let common_args = CommonArgs {
            delimiter: ',',
            infer_types: false,
        };

        let show_args = ShowArgs {
            file_path: "./test-data/test-2.csv".to_string(),
            pretty: true,
            head: false,
            tail: false,
            sort_key: Some("0".to_string()),
            columns: vec!["integer".to_string(), "natural".to_string()],
            start: 0,
            end: usize::MAX,
            filter: None,
            filter_cols: vec![],
            show_row_nums: false,
            dformat: "".to_string(),
            descending: false,
        };

        let result = parse_and_display_csv(&common_args, &show_args);
        assert!(result.is_ok(), "Test failed. Error: {:?}", result.err());
    }

    #[test]
    fn sort_datetime() {
        let common_args = CommonArgs {
            delimiter: ',',
            infer_types: false,
        };

        let show_args = ShowArgs {
            file_path: "./test-data/test-2.csv".to_string(),
            pretty: true,
            head: false,
            tail: false,
            columns: vec![],
            start: 0,
            end: usize::MAX,
            sort_key: Some("date".to_string()),
            dformat: "%d/%m/%Y".to_string(),
            filter: None,
            filter_cols: vec![],
            show_row_nums: false,
            descending: false,
        };

        let result = parse_and_display_csv(&common_args, &show_args);
        assert!(result.is_ok(), "Test failed. Error: {:?}", result.err());
    }

    #[test]
    fn parse_datetime() {
        let date_only = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d").unwrap();
        println!("{}", date_only);
        let raw_string = "07/04/1972";
        let date = NaiveDateTime::parse_from_str(raw_string, "%d/%m/%Y");
        println!("{}", date.unwrap());
    }
}
