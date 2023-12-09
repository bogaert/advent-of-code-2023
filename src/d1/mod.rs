use polars::prelude::*;
use std::error::Error;
use std::fs::read_to_string;
use regex::Regex;

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Read the file
    let file_content = read_to_string("input/d1/input")?;

    // Split the file content into lines
    let lines: Vec<&str> = file_content.lines().collect();

    // Create a Series from the lines
    let series = Series::new("input", lines);

    // Regular expression to find a digit
    let re_digit = Regex::new(r"\d").unwrap();

    // Find the first digit and the last digit in each string
    let first_digits: Vec<Option<i32>> = series.utf8()?
        .into_iter()
        .map(|line| {
            line.and_then(|l| re_digit.find(l)
                .and_then(|m| m.as_str().parse::<i32>().ok()))
        })
        .collect();

    let last_digits: Vec<Option<i32>> = series.utf8()?
        .into_iter()
        .map(|line| {
            line.map(|l| l.chars().rev().collect::<String>())
                .and_then(|rev_l| re_digit.find(&rev_l)
                .and_then(|m| m.as_str().parse::<i32>().ok()))
        })
        .collect();

    // Create Series from the digits
    let first_digit_series = Series::new("first_digit", &first_digits);
    let last_digit_series = Series::new("last_digit", &last_digits);

    // Concatenate first_digit and last_digit into a single integer
    let concatenated_digits: Vec<Option<i32>> = first_digits.into_iter().zip(last_digits.into_iter())
        .map(|(first, last)| {
            match (first, last) {
                (Some(f), Some(l)) => Some(format!("{}{}", f, l).parse::<i32>().unwrap()),
                _ => None
            }
        })
        .collect();

    let concatenated_series = Series::new("concatenated", concatenated_digits);

    // Create a DataFrame from the Series
    let mut df = DataFrame::new(vec![series])?;
    df = df.hstack(&[first_digit_series, last_digit_series, concatenated_series])?;

    // Print the DataFrame to see the output
    println!("{:?}", df);

    // Sum the values in the 'concatenated' column
    let sum: i32 = df.column("concatenated")?
        .i32()?
        .sum()
        .unwrap_or(0);

    // Print the sum
    println!("Sum of concatenated column: {}", sum);

    Ok(())
}
