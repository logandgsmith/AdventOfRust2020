// Advent of Code Day 6 Part 1 (12/06/20)
// Written in Rust!
// Author: Logan D.G. Smith
// Relatively quick solve here on part 1. Glad to use my second favorite data
// structure, the HashSet to complete it.
use std::collections::HashSet;
use advent_of_rust_2020::*;

fn main() {
    // Read from file
    if let Ok(lines) = read_lines("inputs/day6input.txt") {
        // Things to track on each pass
        let mut total_sum = 0;
        let mut group_sum = HashSet::new();

        // Read line by line
        for line in lines {
            if let Ok(entry) = line {
                // Each group is followed by a blank line
                if entry.len() == 0 {
                    total_sum += group_sum.len();
                    let collected = group_sum.drain().collect::<String>();
                    valid_line(&format!("Collected responses: {}", collected));
                }

                // Still in a group, add to the group members
                else {
                    for response in entry.chars() {
                        group_sum.insert(response);
                    }
                }
            }
        }

        // Print Results
        println!("\n~~ Day 6 Part 1 ~~");
        valid_line(&format!("Sum of unique responses: {}", total_sum));
    }
}