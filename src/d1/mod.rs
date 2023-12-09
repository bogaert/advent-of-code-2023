use polars::prelude::*;
use std::error::Error;

fn print_first_5_lines_with_polars() -> Result<(), Box<dyn Error>> {
    // Path to the file in the parent directory
    let path = "input/d1/input";

    // Load the file into a DataFrame
    let df = CsvReader::from_path(path)?
        .infer_schema(None)
        .has_header(false)
        .finish()?;

    // Take the first 5 rows
    let first_5 = df.head(Some(5));

    // Print the DataFrame
    println!("{:?}", first_5);

    Ok(())
}

pub fn solve() {
    if let Err(e) = print_first_5_lines_with_polars() {
        eprintln!("Error: {}", e);
    }
}
