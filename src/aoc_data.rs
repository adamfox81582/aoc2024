use std::fs::File;
use std::io::{self, prelude::*};

pub fn datafile_path(day: u8, is_sample_data: bool) -> String {
    format!("data/day{}.data.{}.txt", day, if is_sample_data {"sample"} else {"real"})
}

pub fn read_tuples_from_file(path: &str) -> io::Result<Vec<(i32, i32)>> {
    // Open the file
    let file = File::open(&path)?;

    // Create a buffer reader for the file
    let reader = io::BufReader::new(file);

    // Create a vector to store the tuples
    let mut tuples: Vec<(i32, i32)> = Vec::new();

    // Read the file line by line
    for line in reader.lines() {
        // Parse the line
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace() // Split by whitespace
            .map(|num| num.parse::<i32>().expect("Invalid number")) // Parse to i32
            .collect();

        // Ensure there are exactly two numbers in the line
        if numbers.len() == 2 {
            tuples.push((numbers[0], numbers[1])); // Add the tuple to the vector
        } else {
            eprintln!("Skipping line with unexpected format: {}", line);
        }
    }

    Ok(tuples)
}

pub fn read_variable_tuples_from_file(path: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut tuples: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace() // Split by whitespace
            .map(|num| num.parse::<i32>().expect("Invalid number")) // Parse to i32
            .collect();
        tuples.push(numbers);
    }

    Ok(tuples)
}
