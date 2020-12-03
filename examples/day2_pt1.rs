// Advent of Code Day 2 Part 1 (12/02/20)
// Written in Rust!
// Author: Logan D.G. Smith
// While the problem was straightforward, I had a few stumbling
// blocks when it came to the text processing. I'm also interested
// in rewriting this with less .unwrap() calls.
use advent_of_rust_2020::read_lines;

fn parse_line(line: &str) -> bool {
    // Seperators
    let lo_sep = line.chars().position(|c| c == '-').unwrap();
    let hi_sep = line.chars().position(|c| c == ' ').unwrap();
    let ps_sep = line.chars().position(|c| c == ':').unwrap();

    // Low and high indexes of 
    let lo_index = line[..lo_sep].parse::<usize>().unwrap();
    let hi_index = line[(lo_sep + 1)..hi_sep].parse::<usize>().unwrap();

    let character: char = line.chars().nth(ps_sep - 1).unwrap();
    let count = line[(ps_sep + 1)..].matches(character).count();

    count >= lo_index && count <= hi_index
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

    println!("Day Two Part One");
    println!("Count: {}", num_valid);
}