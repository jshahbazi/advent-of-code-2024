use std::fs::File;
use std::io::{ self, BufRead, BufReader };
use regex::Regex;

fn evaluate_left_to_right(
    numbers: &[i64],
    current_value: i64,
    idx: usize,
    operators: &[&str],
    results: &mut Vec<i64>,
) {
    if idx == numbers.len() {
        results.push(current_value); 
        return;
    }

    for &op in operators {
        let next_value = match op {
            "+" => current_value + numbers[idx],
            "*" => current_value * numbers[idx],
            "||" => (current_value.to_string() + &numbers[idx].to_string()).parse::<i64>().unwrap(),
            _ => unreachable!(),
        };
        evaluate_left_to_right(numbers, next_value, idx + 1, operators, results);
    }
}
 
fn part(first: &i64, numbers: Vec<i64>) -> i64 {
    let operators = vec!["+", "*", "||"];
    let mut results = Vec::new();

    evaluate_left_to_right(&numbers[1..], numbers[0], 0, &operators, &mut results);

    if results.contains(first) {
        *first
    } else {
        0
    }
}

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let line_pattern = Regex::new(r"^(?P<first>\d+):(?P<rest>(?:\s\d+)+)$").unwrap();

    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;
        for caps in line_pattern.captures_iter(&line) {
            let mut first: i64 = 0;
            let mut numbers: Vec<i64> = Vec::new();
            if let Some(result) = caps.name("first") {
                first = result.as_str().parse().unwrap_or(0);
            }
            if let Some(rest) = caps.name("rest") {
                numbers = rest.as_str().split_whitespace().map(|s| s.parse::<i64>().unwrap_or(0)).collect();
            }

            sum += part(&first, numbers);
        }      
    }


    println!("Sum: {}", sum);

    Ok(())
}
