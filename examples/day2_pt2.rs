// Advent of Code Day 2 Part 2 (12/02/20)
// Written in Rust!
// Author: Logan D.G. Smith
use advent_of_rust_2020::read_lines;

fn parse_line(line: &str) -> bool {
    // Seperators
    let lo_sep = line.chars().position(|c| c == '-').unwrap();
    let hi_sep = line.chars().position(|c| c == ' ').unwrap();
    let ps_sep = line.chars().position(|c| c == ':').unwrap();

    // Low and high indexes of 
    let lo_index = line[..lo_sep].parse::<usize>().unwrap();
    let hi_index = line[(lo_sep + 1)..hi_sep].parse::<usize>().unwrap();
    
    // Information on valid password
    let password = &line[(ps_sep + 2)..];
    let character: char = line.chars().nth(ps_sep - 1).unwrap();
    
    // Actual character at the index location
    let index_one: char = password.chars().nth(lo_index - 1).unwrap();
    let index_two: char = password.chars().nth(hi_index - 1).unwrap();

    // println!("1: {} {}, 2: {} {}", lo_index, index_one, hi_index, index_two);

    // Password is valid iif exactly one index matches the character
    (index_one == character) ^ (index_two == character)
}

fn main()  {
    let mut num_valid = 0; // Keep track of how many valid passwords we've seen

    // Check if file exists
    if let Ok(lines) = read_lines("inputs/day2input.txt") {
        // Read line by line
        for line in lines {
            if let Ok(entry) = line {
                // Skip empty strings
                if entry.len() == 0 {
                    continue;
                }
                // Parse for validity
                if parse_line(&entry) {
                    num_valid += 1;
                }
            }
            else {
                println!("Failed to parse");
            }
        }
    }

    println!("Day Two Part Two");
    println!("Count: {}", num_valid);
}