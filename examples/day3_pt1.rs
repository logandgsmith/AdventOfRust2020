// Advent of Code Day 3 Part 1 (12/03/20)
// Written in Rust!
// Author: Logan D.G. Smith
// While this part of today's challenges was fairly easy,
// I thought the exercise was interesting. Can probably be
// optimized more.
use advent_of_rust_2020::read_lines;

fn main() {
    let mut current_col = 0; // columns wrap
    let mut num_trees = 0;   // Keep track of how many trees ('#') are encountered

    // Check if file exists
    if let Ok(lines) = read_lines("inputs/day3input.txt") {
        // Read line by line
        for line in lines {
            if let Ok(entry) = line {
                let length = entry.len();
                // Skip empty strings
                if length == 0 {
                    continue;
                }

                // Parse for trees
                if entry.chars().nth(current_col).unwrap() == '#' {
                    num_trees += 1;
                }

                //Move 3 cols to the right. Loop if needed
                current_col = (current_col + 3) % length;              
            }
            else {
                println!("Failed to parse");
            }
        }
    }

    println!("{}", num_trees);
}