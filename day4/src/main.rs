use std::fs::File;
use std::io::{self, BufRead, BufReader};

use regex::Regex;

fn get_top_left_diagonals(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = grid.len();
    let m = grid[0].len();
    let mut diagonals = Vec::new();

    // Diagonals starting from the first row
    for col in 0..m {
        let mut diagonal = Vec::new();
        let mut row = 0;
        let mut c = col;
        while row < n && c < m {
            diagonal.push(grid[row][c]);
            row += 1;
            c += 1;
        }
        diagonals.push(diagonal);
    }

    // Diagonals starting from the first column (excluding the first element)
    for row_start in 1..n {
        let mut diagonal = Vec::new();
        let mut row = row_start;
        let mut c = 0;
        while row < n && c < m {
            diagonal.push(grid[row][c]);
            row += 1;
            c += 1;
        }
        diagonals.push(diagonal);
    }

    diagonals
}

fn get_top_right_diagonals(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = grid.len();
    let m = grid[0].len();
    let mut diagonals = Vec::new();

    // Diagonals starting from the first row (from right to left)
    for col in (0..m).rev() {
        let mut diagonal = Vec::new();
        let mut row = 0;
        let mut c = col as isize;
        while row < n && c >= 0 {
            diagonal.push(grid[row][c as usize]);
            row += 1;
            c -= 1;
        }
        diagonals.push(diagonal);
    }

    // Diagonals starting from the last column of each subsequent row
    for row_start in 1..n {
        let mut diagonal = Vec::new();
        let mut row = row_start;
        let mut c = (m - 1) as isize;
        while row < n && c >= 0 {
            diagonal.push(grid[row][c as usize]);
            row += 1;
            c -= 1;
        }
        diagonals.push(diagonal);
    }

    diagonals
}



fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    // let xmas_pattern = Regex::new(r"(?P<XMAS>XMAS)").unwrap();

    let xmas_pattern = vec![
        vec![Some('M'), None, Some('S')],
        vec![None, Some('A'), None],
        vec![Some('M'), None, Some('S')],
    ];

    let reverse_xmas_pattern = vec![
        vec![Some('S'), None, Some('M')],
        vec![None, Some('A'), None],
        vec![Some('S'), None, Some('M')],
    ];    

    let top_down_xmas_pattern = vec![
        vec![Some('M'), None, Some('M')],
        vec![None, Some('A'), None],
        vec![Some('S'), None, Some('S')],
    ];

    let bottom_up_xmas_pattern = vec![
        vec![Some('S'), None, Some('S')],
        vec![None, Some('A'), None],
        vec![Some('M'), None, Some('M')],
    ];        

    let mut sum: i32 = 0;

    let mut grid: Vec<Vec<char>> = Vec::new();
    // let mut vec_of_strings: Vec<String> = Vec::new();
    for line in reader.lines() {
        // vec_of_strings.push(line.as_ref().unwrap().clone());
        // vec_of_strings.push(line.as_ref().unwrap().chars().rev().collect());

        let mut line_vector = Vec::new();
        for character in line?.chars() {
            line_vector.push(character);
        }
        // let rev_line_vector = line_vector.clone().reverse();
        grid.push(line_vector);
    }

    let grid_height = grid.len();
    let grid_width = grid[0].len();
    let pattern_height = xmas_pattern.len();
    let pattern_width = xmas_pattern[0].len();


    for i in 0..=grid_height - pattern_height {
        for j in 0..=grid_width - pattern_width {
            let mut match_found = true;

            for (p_i, pattern_row) in xmas_pattern.iter().enumerate() {
                for (p_j, pattern_cell) in pattern_row.iter().enumerate() {
                    match pattern_cell {
                        Some(c) => {
                            if grid[i + p_i][j + p_j] != *c {
                                match_found = false;
                                break;
                            }
                        }
                        None => continue, // Wildcard matches any character
                    }
                }
                if !match_found {
                    break;
                }
            }
            if match_found {
                sum += 1;
                // println!("Pattern found at position ({}, {})", i, j);
            }

            match_found = true;
            for (p_i, pattern_row) in reverse_xmas_pattern.iter().enumerate() {
                for (p_j, pattern_cell) in pattern_row.iter().enumerate() {
                    match pattern_cell {
                        Some(c) => {
                            if grid[i + p_i][j + p_j] != *c {
                                match_found = false;
                                break;
                            }
                        }
                        None => continue, // Wildcard matches any character
                    }
                }
                if !match_found {
                    break;
                }
            }
            if match_found {
                sum += 1;
                // println!("Reverse pattern found at position ({}, {})", i, j);
            }   

            match_found = true;
            for (p_i, pattern_row) in top_down_xmas_pattern.iter().enumerate() {
                for (p_j, pattern_cell) in pattern_row.iter().enumerate() {
                    match pattern_cell {
                        Some(c) => {
                            if grid[i + p_i][j + p_j] != *c {
                                match_found = false;
                                break;
                            }
                        }
                        None => continue, // Wildcard matches any character
                    }
                }
                if !match_found {
                    break;
                }
            }
            if match_found {
                sum += 1;
                // println!("Reverse pattern found at position ({}, {})", i, j);
            }    

            match_found = true;
            for (p_i, pattern_row) in bottom_up_xmas_pattern.iter().enumerate() {
                for (p_j, pattern_cell) in pattern_row.iter().enumerate() {
                    match pattern_cell {
                        Some(c) => {
                            if grid[i + p_i][j + p_j] != *c {
                                match_found = false;
                                break;
                            }
                        }
                        None => continue, // Wildcard matches any character
                    }
                }
                if !match_found {
                    break;
                }
            }
            if match_found {
                sum += 1;
                // println!("Reverse pattern found at position ({}, {})", i, j);
            }                                 
        }
    }    

    // for col in 0..grid[0].len() {
    //     let mut word = String::new();
    //     for row in &grid {
    //         word.push(row[col]);
    //     }
    //     vec_of_strings.push(word.clone());
    //     vec_of_strings.push(word.chars().rev().collect());
    // }
    
    // let top_left_diagonals = get_top_left_diagonals(&grid);
    // let top_right_diagonals = get_top_right_diagonals(&grid);

    // for diagonal in top_left_diagonals {
    //     let word: String = diagonal.iter().collect();
    //     vec_of_strings.push(word.clone());
    //     let reversed_word: String = word.chars().rev().collect();
    //     vec_of_strings.push(reversed_word);
    // }

    // for diagonal in top_right_diagonals {
    //     let word: String = diagonal.iter().collect();
    //     vec_of_strings.push(word.clone());
    //     let reversed_word: String = word.chars().rev().collect();
    //     vec_of_strings.push(reversed_word);
    // }

    // for line in vec_of_strings {
    //     let text = line;
    //     for caps in xmas_pattern.captures_iter(&text) {
    //         if let Some(_xmas_match) = caps.name("XMAS") {
    //             sum += 1;
    //         }
    //     }
    // }

    println!("Sum: {}", sum);

    Ok(())
}
