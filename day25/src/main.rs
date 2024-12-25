use std::fs::File;
use std::io::{ self, BufRead, BufReader };
use std::collections::{VecDeque};
use regex::Regex;



fn part1(locks: VecDeque<Vec<usize>>, keys: VecDeque<Vec<usize>>) -> i32 {
    let mut count: i32 = 0;
    let target = vec![5, 5, 5, 5, 5];

    for lock in locks.iter() {
        for key in keys.iter() {
            let all_within_target = lock
                .iter()
                .zip(key.iter())
                .zip(target.iter())
                .all(|((l, k), t)| l + k <= *t);

            if all_within_target {
                // println!("Lock {:?} and Key {:?} are within target {:?}", lock, key, target);
                count += 1;
            } 
        }
    }


    count
}


fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    // let mut wiring_values: HashMap<String, usize> = HashMap::new();
    let mut locks: VecDeque<Vec<usize>> = VecDeque::new();
    let mut keys: VecDeque<Vec<usize>> = VecDeque::new();

    let all_hash = Regex::new(r"^#{5}$").unwrap();
    let all_period = Regex::new(r"^\.{5}$").unwrap();

    let mut lines_iter = reader.lines();

    while let Some(group) = (0..7).map(|_| lines_iter.next()).collect::<Option<Vec<_>>>() {
        let lines: Vec<String> = group.into_iter().collect::<Result<_, _>>()?;
        let trimmed_lines: Vec<&str> = lines.iter().map(|line| line.trim()).collect();

        // Check the first and last lines (after trimming)
        if let (Some(first), Some(last)) = (trimmed_lines.first(), trimmed_lines.last()) {
            // println!("First line: {}", first);
            // println!("Last line: {}", last);

            if all_hash.is_match(first) && all_period.is_match(last) {
                // println!("Lock!");
                let mut lock_values: Vec<usize> = vec![0,0,0,0,0];
                let subset = &trimmed_lines[1..6]; // Get lines 2 through 6
                for thing_line in subset{
                    // println!("{}", thing_line);
                    thing_line.chars().enumerate().for_each(|(j, ch)| {
                        if ch == '#' { lock_values[j] += 1 }
                    });                    
                }
                // println!("{:?}", lock_values);
                locks.push_back(lock_values);
            } else if all_period.is_match(first) && all_hash.is_match(last) {
                // println!("Key!");
                let mut key_values: Vec<usize> = vec![0,0,0,0,0];
                let subset = &trimmed_lines[1..6]; // Get lines 2 through 6
                for thing_line in subset{
                    // println!("{}", thing_line);
                    thing_line.chars().enumerate().for_each(|(j, ch)| {
                        if ch == '#' { key_values[j] += 1 }
                    });                    
                }
                // println!("{:?}", key_values);
                keys.push_back(key_values);                
            }
        }

        // Check if the next line is empty
        if let Some(Ok(next_line)) = lines_iter.next() {
            if !next_line.trim().is_empty() {
                println!("Error: The next line is not empty: {}", next_line.trim());
            }
        } else {
            // done with the file
            break;
        }
    }

    let result = part1(locks, keys);
    println!("{}", result);


    // println!("{:?}", locks);
        // for caps in gate_pattern.captures_iter(&line) {
        //     let first_wire = caps.name("first_wire").unwrap().as_str().to_string();
        //     let operation = caps.name("operation").unwrap().as_str().to_string();
        //     let second_wire = caps.name("second_wire").unwrap().as_str().to_string();
        //     let output_wire = caps.name("output_wire").unwrap().as_str().to_string();            
        //     // println!("{:?} {:?} {:?} -> {:?}", first_wire, operation, second_wire, output_wire);
        //     // calculate_operation(&wiring_values, first_wire, operation, second_wire);
        //     wiring_queue.push_back((first_wire.clone(), operation.clone(), second_wire.clone(), output_wire.clone()));
        // }
    // }

    // let result_part1 = part1(wiring_values, wiring_queue);



    Ok(())
}
