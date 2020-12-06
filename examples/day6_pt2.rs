// Advent of Code Day 6 Part 2 (12/06/20)
// Written in Rust!
// Author: Logan D.G. Smith
// Brought back the HashMap for part two in order to keep track of how 
// many occurences of each response were found. Spent a bit longer than
// I'd like to admit trying to think of the best way to solve this.
use std::collections::HashMap;
use advent_of_rust_2020::*;

fn main() {
    // Read from file
    if let Ok(lines) = read_lines("inputs/day6input.txt") {
        // Info to track from each group
        let mut total_sum = 0;
        let mut group_mem = 0;
        let mut group_sum: HashMap<char, i32> = HashMap::new();

        // Read line by line
        for line in lines {
            if let Ok(entry) = line {
                // Each group is followed by a blank line
                if entry.len() == 0 {
                    let mut valid_responses = 0;

                    // If everyone in a group responded yes, add the response.
                    for value in group_sum.values() {
                        if *value == group_mem { valid_responses += 1; }
                    }

                    // Print how many responses found and clean up
                    total_sum += valid_responses;
                    valid_line(&format!("Responses: {}", valid_responses));
                    group_mem = 0;
                    group_sum.drain();
                }

                // If we're still in a group, parse for responses
                else {
                    group_mem += 1;
                    for response in entry.chars() {
                        // Add new value to the seen list
                        if !group_sum.contains_key(&response) {
                            group_sum.insert(response, 1);
                        }

                        // If we have seen it, add to its match count
                        else {
                            group_sum.insert(response, group_sum.get(&response).unwrap() + 1);
                        }
                    }
                }
            }
        }

        // Print Results
        println!("\n~~ Day 6 Part 2 ~~");
        valid_line(&format!("Sum of unique responses: {}", total_sum));
    }
}