use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn count_digits(mut n: u64) -> usize {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}

// Transform one token according to your rules, returning a list of new tokens.
fn transform(token: &str) -> Vec<String> {
    let mut results = Vec::new();
    match token.parse::<u64>() {
        Ok(0) => {
            // "0" -> "1"
            results.push("1".into());
        }
        Ok(digit) if count_digits(digit) % 2 == 0 => {
            // Even number of digits -> potentially split
            let s = digit.to_string();
            if s.len() > 1 {
                let center = s.len() / 2;
                let (left, right) = s.split_at(center);

                // Possibly trim leading zeros on the right half
                if let Ok(r) = right.parse::<u64>() {
                    if count_digits(r) > 1 {
                        // If right side has more than 1 digit, trim leading zeros
                        let trimmed = right.trim_start_matches('0');
                        results.push(left.to_string());
                        results.push(trimmed.to_string());
                    } else {
                        // Single-digit right half
                        results.push(left.to_string());
                        results.push(right.to_string());
                    }
                } else {
                    // parse error for right side
                    results.push(left.to_string());
                    results.push("?".to_string());
                }
            } else {
                // Single even-digit number
                results.push(s);
            }
        }
        Ok(digit) => {
            // Odd number of digits -> multiply by 2024
            let multiplied = digit * 2024;
            results.push(multiplied.to_string());
        }
        Err(_) => {
            // parse error
            results.push("?".to_string());
        }
    }
    results
}

fn main() -> Result<(), Error> {
    // Example: reading your starting line from a file
    let file = File::open("./input.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    
    // Build initial frequency map
    let mut freq_map = HashMap::<String, u64>::new();
    for token in line.split_whitespace() {
        *freq_map.entry(token.to_string()).or_insert(0) += 1;
    }

    // Run 75 iterations
    for _ in 0..75 {
        let mut new_map = HashMap::<String, u64>::new();
        for (token, count) in freq_map.into_iter() {
            // Transform each unique token once
            let results = transform(&token);
            // For each result, add 'count' occurrences
            for r in results {
                *new_map.entry(r).or_insert(0) += count;
            }
        }
        freq_map = new_map;
    }

    // The final count is the sum of all frequencies
    let total_count: u64 = freq_map.values().sum();
    println!("count: {}", total_count);

    Ok(())
}
