use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn is_sequence_safe(values: &Vec<i32>) -> bool {
    if values.len() < 2 {
        return true; // A single value is safe by default
    }

    let mut increasing = None;
    for i in 0..values.len() - 1 {
        let diff = values[i + 1] - values[i];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        let is_increasing = diff > 0;
        if increasing.is_none() {
            increasing = Some(is_increasing);
        } else if increasing.unwrap() != is_increasing {
            return false;
        }
    }
    true
}

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut safe_reports = 0;
    let mut unsafe_reports = 0;

    for line in reader.lines() {
        let line = line?;
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let mut levels_removed = 0;

        if is_sequence_safe(&values) {
            safe_reports += 1;
            continue;
        } else {
            let mut found_safe = false;
            for i in 0..values.len() {
                let mut modified_values = values.clone();
                modified_values.remove(i);

                if is_sequence_safe(&modified_values) {
                    levels_removed = 1;
                    safe_reports += 1;
                    found_safe = true;
                    println!(
                        "Removed value at index {}: {} to make line safe",
                        i, values[i]
                    );
                    break;
                }
            }
            if !found_safe {
                unsafe_reports += 1;
                println!("Line cannot be made safe by removing one element: {}", line);
            }
        }
    }

    println!("Safe reports: {}", safe_reports);
    println!("Unsafe reports: {}", unsafe_reports);

    Ok(())
}
