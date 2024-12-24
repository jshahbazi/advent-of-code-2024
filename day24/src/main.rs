use std::fs::File;
use std::io::{ self, BufRead, BufReader };
use std::collections::{HashMap, VecDeque};
use regex::Regex;


fn calculate_operation(wiring_values: &HashMap<String, usize>, first_wire: &str, operation: &str, second_wire: &str) -> usize {
    if let Some(&value) = wiring_values.get(first_wire) {
        println!("Value for {}: {}", first_wire, value);
    } else {
        println!("Key not found! {:?}", first_wire);
    }

    0
}


fn part1(mut wiring_values: HashMap<String, usize>, mut wiring_queue: VecDeque<(String, String, String, String)>) -> i32 {
    // println!("{:?}",wiring_values);
    while let Some(value) = wiring_queue.pop_front() {
        let first_wire = &value.0;
        let operation = &value.1;
        let second_wire = &value.2;
        let output_wire = &value.3;

        if let (Some(&first_wire_value), Some(&second_wire_value)) = (
            wiring_values.get(first_wire.as_str()),
            wiring_values.get(second_wire),
        ) {
            // println!("First: {}, Second: {}", first_wire_value, second_wire_value);
            let value = match operation.as_str() {
                "XOR" => first_wire_value ^ second_wire_value,
                "OR" => first_wire_value | second_wire_value,
                "AND" => first_wire_value & second_wire_value,
                _ => 0,
            };
            wiring_values.insert(output_wire.to_string(), value);
        } else {
            wiring_queue.push_back(value);
        }
    }

    // println!("{:?}", wiring_values);

    let mut filtered_and_sorted: Vec<_> = wiring_values
        .iter()
        .filter(|(key, _)| key.starts_with('z'))
        .collect();

    // Sort by the keys, interpreting the numeric part as an integer
    filtered_and_sorted.sort_by_key(|(key, _)| key[1..].parse::<usize>().unwrap());

    for (key, value) in &filtered_and_sorted {
        println!("Key: {}, Value: {}", key, value);
    }

    filtered_and_sorted.reverse();
    let binary_number: String = filtered_and_sorted
    .iter()
    .map(|(_, &value)| value.to_string()) // Convert each value to a string
    .collect();

    println!("Binary number: {}", binary_number);
    let bnumber = usize::from_str_radix(&binary_number, 2).unwrap();
    println!("Part 1 Number: {}", bnumber);

    0
}

// fn part2(first: &i64, numbers: Vec<i64>) -> i64 {

// }

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut wiring_values: HashMap<String, usize> = HashMap::new();
    let mut wiring_queue: VecDeque<(String, String, String, String)> = VecDeque::new();

    let wire_pattern = Regex::new(r"^(?P<wire_input>\w{3}):\s*(?P<wire_value>\d+)$").unwrap();
    let gate_pattern = Regex::new(r"^(?P<first_wire>\w{3})\s+(?P<operation>XOR|OR|AND)\s+(?P<second_wire>\w{3})\s+->\s+(?P<output_wire>\w+)$").unwrap();
    


    for line in reader.lines() {
        let line = line?;
        for caps in wire_pattern.captures_iter(&line) {
            let wire_input = caps.name("wire_input").unwrap().as_str();
            let wire_value = caps.name("wire_value").unwrap().as_str();
            wiring_values.insert(wire_input.to_string(), wire_value.parse::<usize>().unwrap());
        }


        for caps in gate_pattern.captures_iter(&line) {
            let first_wire = caps.name("first_wire").unwrap().as_str().to_string();
            let operation = caps.name("operation").unwrap().as_str().to_string();
            let second_wire = caps.name("second_wire").unwrap().as_str().to_string();
            let output_wire = caps.name("output_wire").unwrap().as_str().to_string();            
            // println!("{:?} {:?} {:?} -> {:?}", first_wire, operation, second_wire, output_wire);
            // calculate_operation(&wiring_values, first_wire, operation, second_wire);
            wiring_queue.push_back((first_wire.clone(), operation.clone(), second_wire.clone(), output_wire.clone()));
        }
    }

    let result_part1 = part1(wiring_values, wiring_queue);



    Ok(())
}
