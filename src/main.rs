use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn print_first_5_lines_of_file() -> io::Result<()> {
    // Path to the file in the parent directory
    let path = Path::new("input/d1/input");

    // Open the file
    let file = File::open(&path)?;

    // Use a buffered reader to read the file line-by-line
    let reader = io::BufReader::new(file);

    // Iterate over the lines and print the first 5
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        println!("{}", line);

        // Stop after 5 lines
        if index >= 4 {
            break;
        }
    }

    Ok(())
}

fn main() {
    // Call the function in main
    if let Err(e) = print_first_5_lines_of_file() {
        eprintln!("Error reading file: {}", e);
    }
}
