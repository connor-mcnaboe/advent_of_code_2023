use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn sum_calibration_values() -> i32 {
    let mut calibration_sum: i32 = 0;
    // iterate over calibration.txt
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./src/calibration.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(code) = line {
                // For each value, find the first and last digit in the code
                let numbers = extract_integers(&code);

                // Combine the first and last digit into a single number
                let first = numbers[0];
                let last = numbers[numbers.len() -1];
                let combined_codes = combine_integers(first, last);
                // Increment summation var
                calibration_sum = calibration_sum + combined_codes;
            }
        }
    }
    calibration_sum
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn extract_integers(s: &str) -> Vec<i32> {
    let mut numbers = Vec::new();

    for c in s.chars() {
        if c.is_digit(10) {
            let current_number = c.to_digit(10).unwrap() as i32;
            numbers.push(current_number);
        }
    }

    numbers
}

fn combine_integers(a: i32, b: i32) -> i32 {
    let combined = format!("{}{}", a, b);
    combined.parse::<i32>().unwrap()
}