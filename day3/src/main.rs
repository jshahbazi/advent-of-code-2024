use std::fs::File;
use std::io::{self, BufRead, BufReader};

use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let pattern = r"mul\((\d+),(\d+)\)"; 

    let re = Regex::new(pattern).unwrap();

    let mut sum: i32 = 0;

    for line in reader.lines() {
        let text = line?;
        for caps in re.captures_iter(&text) {
            let num1: i32 = caps[1].parse().unwrap();
            let num2: i32 = caps[2].parse().unwrap();

            sum += num1 * num2;
        }
    }

    println!("Sum: {}", sum);

    Ok(())
}
