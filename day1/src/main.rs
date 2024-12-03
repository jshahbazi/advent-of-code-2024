use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        let parts: Vec<&str> = line.split_whitespace().collect();

        if let (Some(first), Some(second)) = (parts.first(), parts.get(1)) {
            let num1: i32 = first.parse().expect("Failed to parse first number");
            let num2: i32 = second.parse().expect("Failed to parse second number");
            
            left_list.push(num1);
            right_list.push(num2);
        } else {
            eprintln!("Error: Line does not contain two numbers");
        }        
    }

    let mut similarity_score = 0;
    for value in left_list.clone() {
        similarity_score += value * right_list.iter().filter(|&&x| x == value).count() as i32;
    }
    println!("Similarity score: {}", similarity_score);



    let mut distance_sum = 0;
    left_list.sort_by(|a, b| b.cmp(a));
    right_list.sort_by(|a, b| b.cmp(a));
    while let (Some(left_value), Some(right_value)) = (left_list.pop(), right_list.pop()) {
        distance_sum += (left_value - right_value).abs();
    }
    println!("Sum of distances: {}", distance_sum);
    
    Ok(())
}
