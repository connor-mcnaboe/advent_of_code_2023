use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn sum_calibration_values(is_pt_2: bool) -> i32 {
    let mut calibration_sum: i32 = 0;
    // iterate over calibration.txt
    if let Ok(lines) = read_lines("./src/calibration.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(code) = line {
                // For each value, find the first and last digit in the code
                let numbers = if is_pt_2 {
                    extract_digits_pt_2(&code)
                } else {
                    extract_digits(&code)
                };

                // Combine the first and last digit into a single number
                let first = numbers[0];
                let last = numbers[numbers.len() - 1];
                let combined_codes = combine_integers(first, last);
                // Increment summation var
                calibration_sum = calibration_sum + combined_codes;
            }
        }
    }
    calibration_sum
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn extract_digits(s: &str) -> Vec<i32> {
    let mut numbers = Vec::new();

    for c in s.chars() {
        if c.is_digit(10) {
            let current_number = c.to_digit(10).unwrap() as i32;
            numbers.push(current_number);
        }
    }

    numbers
}

fn extract_digits_pt_2(s: &str) -> Vec<i32> {
    let map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut numbers = Vec::new();
    let mut digit_string: String = String::new();
    for c in s.chars() {
        if c.is_digit(10) {
            let current_number = c.to_digit(10).unwrap() as i32;
            numbers.push(current_number);
            digit_string = "".parse().unwrap();
        } else {
            digit_string.push(c);
            for key in map.keys() {
                if digit_string.contains(key) {
                    let digit = map.get(&*key).unwrap();
                    numbers.push(*digit);
                    // Keep the last character in case it starts a new string, ex. twone or threeight
                    digit_string = c.to_string();
                    break;
                }
            }
        }
    }
    numbers
}

fn combine_integers(a: i32, b: i32) -> i32 {
    let combined = format!("{}{}", a, b);
    combined.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt_2() {
        let mut sum = 0;
        let strings = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
            "twone",
        ];
        for s in strings {
            let digits = extract_digits_pt_2(s);
            let first = digits[0];
            let last = digits[digits.len() - 1];
            let combined_codes = combine_integers(first, last);
            // Increment summation var
            sum = sum + combined_codes;
        }
        assert_eq!(302, sum);
    }
}
