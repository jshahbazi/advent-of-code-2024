use std::fs::File;
use std::io::{ self, BufRead, BufReader };
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Machine {
    id: String,          // Unique identifier for the machine
    button_a: (i32, i32), // Coordinates for Button A
    button_b: (i32, i32), // Coordinates for Button B
    prize: (i32, i32),    // Coordinates for the prize
}

impl Machine {
    pub fn calculate_tokens_to_win(&self) -> i32 {
        // self.button_a.0 + self.button_a.1 + self.button_b.0 + self.button_b.1 + self.prize.0 + self.prize.1
        let x1 = self.button_a.0;
        let y1 = self.button_a.1;
        let x2 = self.button_b.0;
        let y2 = self.button_b.1;
        let px = self.prize.0;        
        let py = self.prize.1;
        
        for A in 1..=100 {
            for B in 1..=100 {
                if x1*A + x2*B == px && y1*A + y2*B == py {
                    return A*3 + B;
                }
            }
        }

        0
    }
}



fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let button_a_pattern = Regex::new(r"^Button\s+A:\s+X.(?P<button_a_x>\d+),\s+Y.(?P<button_a_y>\d+)$").unwrap();
    let button_b_pattern = Regex::new(r"^Button\s+B:\s+X.(?P<button_b_x>\d+),\s+Y.(?P<button_b_y>\d+)$").unwrap();
    let prize_pattern = Regex::new(r"^Prize:\s+X.(?P<prize_x>\d+),\s+Y.(?P<prize_y>\d+)$").unwrap();

    let mut all_machines: Vec<Machine> = Vec::new();
    
    let mut button_a_x = 0;
    let mut button_a_y = 0;
    let mut button_b_x = 0;
    let mut button_b_y = 0;
    let mut prize_x = 0;
    let mut prize_y = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        match index % 4 {
            0 => if let Some(caps) = button_a_pattern.captures(&line) {
                button_a_x = caps.name("button_a_x").unwrap().as_str().parse::<i32>().unwrap();
                button_a_y = caps.name("button_a_y").unwrap().as_str().parse::<i32>().unwrap();
            },
            1 =>if let Some(caps) = button_b_pattern.captures(&line) {
                // println!("{} {}", button_a_x, button_a_y);
                button_b_x = caps.name("button_b_x").unwrap().as_str().parse::<i32>().unwrap();
                button_b_y = caps.name("button_b_y").unwrap().as_str().parse::<i32>().unwrap();
            },
            2 =>if let Some(caps) = prize_pattern.captures(&line) {
                prize_x = caps.name("prize_x").unwrap().as_str().parse::<i32>().unwrap();
                prize_y = caps.name("prize_y").unwrap().as_str().parse::<i32>().unwrap();
                // println!("{} {}", prize_x, prize_y);
            },

            3 => {
                let new_machine = Machine {
                    id: index.to_string(),
                    button_a: (button_a_x, button_a_y),
                    button_b: (button_b_x, button_b_y),
                    prize: (prize_x, prize_y),
                };
                // println!("{:?}", new_machine);
                all_machines.push(new_machine);
            },
            _ => {todo!()}
        }

    }

    let mut total_tokens = 0;

    for thing in all_machines{
        // println!("{:?}", thing);
        // println!("{:?}", thing.calculate_tokens_to_win());
        total_tokens += thing.calculate_tokens_to_win();
    }

    println!("{}", total_tokens);


    Ok(())
}
