use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

fn topological_sort(numbers: Vec<i32>, rules: HashMap<i32, Vec<i32>>) -> Result<Vec<i32>, &'static str> {
    let numbers_set: HashSet<i32> = numbers.iter().cloned().collect();

    // Filter rules so that both keys and values appear in `numbers`
    let filtered_rules: HashMap<i32, Vec<i32>> = rules.into_iter().filter_map(|(k, vs)| {
        if numbers_set.contains(&k) {
            let filtered_vs: Vec<i32> = vs.into_iter().filter(|v| numbers_set.contains(v)).collect();
            Some((k, filtered_vs))
        } else {
            None
        }
    }).collect();

    // All nodes are just `numbers`
    let all_nodes: Vec<i32> = numbers;

    // Build adjacency list and in-degree map
    let mut adjacency: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut in_degree: HashMap<i32, usize> = HashMap::new();

    for &node in &all_nodes {
        adjacency.insert(node, Vec::new());
        in_degree.insert(node, 0);
    }

    // Construct edges
    // Assuming: If rule says k -> [v1, v2], that means k must come BEFORE v1 and v2.
    // If this interpretation is wrong, switch the direction of edges.
    for (k, vs) in filtered_rules {
        for v in vs {
            adjacency.get_mut(&k).unwrap().push(v);
            *in_degree.get_mut(&v).unwrap() += 1;
        }
    }

    let mut queue = VecDeque::new();
    for (&node, &deg) in &in_degree {
        if deg == 0 {
            queue.push_back(node);
        }
    }

    let mut sorted = Vec::new();

    while let Some(node) = queue.pop_front() {
        sorted.push(node);

        for &neighbor in &adjacency[&node] {
            let deg = in_degree.get_mut(&neighbor).unwrap();
            *deg -= 1;
            if *deg == 0 {
                queue.push_back(neighbor);
            }
        }
    }

    if sorted.len() == all_nodes.len() {
        Ok(sorted)
    } else {
        Err("A cycle was detected - no valid topological ordering exists.")
    }
}

fn find_middle(numbers: &mut Vec<i32>) -> Option<i32> {
    if numbers.is_empty() {
        return None; 
    }
    // numbers.sort();
    let len = numbers.len();
    let middle = Some(numbers[len / 2]).unwrap();

    // println!("Numbers: {:?}", numbers);
    // println!("Middle: {}", middle);
    Some(middle)
}


fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let rules_pattern = Regex::new(r"(?P<num1>\d+)\|(?P<num2>\d+)").unwrap();

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut sum: i32 = 0;

    for line in reader.lines() {
        let line = line?;
        for caps in rules_pattern.captures_iter(&line) {
            let num1: i32 = caps["num1"].parse().unwrap();
            let num2: i32 = caps["num2"].parse().unwrap();
            rules.entry(num1)
            .or_default()  // Creates a new Vec if the key doesn't exist
            .push(num2);                
            
        }

        let mut good: bool = true;
        if line.contains(",") {
            let update_line: Vec<&str> = line.split(",").collect();
            let mut numbers: Vec<i32> = update_line.iter().map(|x| x.parse().unwrap()).collect();
            
            for (i, &current_page) in update_line.iter().enumerate() {
                // Parse the current page number
                if let Ok(current_page_number) = current_page.parse::<i32>() {
                    // Iterate over the remaining numbers
                    for &next_page in update_line.iter().skip(i + 1) {
                        // Parse the next page number
                        if let Ok(next_page_number) = next_page.parse::<i32>() {
                            // Lookup the current page in the hashmap
                            if let Some(vec) = rules.get(&current_page_number) {
                                // Check if the vector contains the next page number
                                if vec.contains(&next_page_number) {
                                    // println!("{} is in the vector associated with key '{}'.", next_page_number, current_page_number);
                                } else {
                                    // println!("{} is NOT in the vector associated with key '{}' - {:?}.", next_page_number, current_page_number,vec);
                                    good = false;
                                    break;
                                }
                            } else {
                                println!("Key '{}' does not exist in the hashmap.", current_page_number);
                            }
                        }
                    }
                }
            }

            if !good {
                // println!("Good: {:?}", numbers);
                println!("Bad: {:?}", numbers);
                match topological_sort(numbers.clone(), rules.clone()) {
                    Ok(sorted) => numbers = sorted,
                    Err(err) => println!("Error: {}", err),
                }
                println!("Bad: {:?}", numbers);
                let median = find_middle(&mut numbers).unwrap();
                println!("Median: {}", median);
                sum += median;
            }            
        }
        

        

    }


    println!("Sum: {}", sum);

    Ok(())
}
