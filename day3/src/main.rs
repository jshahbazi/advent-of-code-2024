use std::fs::File;
use std::io::{self, BufRead, BufReader};

use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let pattern = Regex::new(r"(?P<mul>mul\((?P<num1>\d+),(?P<num2>\d+)\))|(?P<do>do\(\))|(?P<dont>don\'t\(\))").unwrap();

    let mut sum: i32 = 0;
    let mut enabled: bool = true;

    for line in reader.lines() {
        let text = line?;
        for caps in pattern.captures_iter(&text) {
            if let Some(mul_match) = caps.name("mul") {
                if enabled {
                    println!("Matched: {}", mul_match.as_str());
                    let num1: i32 = caps["num1"].parse().unwrap();
                    let num2: i32 = caps["num2"].parse().unwrap();
                    sum += num1 * num2;
                }
            } else if let Some(do_match) = caps.name("do") {
                println!("Matched: {}", do_match.as_str());
                enabled = true;
            } else if let Some(dont_match) = caps.name("dont") {
                println!("Matched: {}", dont_match.as_str());
                enabled = false;
            }
        }
    }

    println!("Sum: {}", sum);

    Ok(())
}
