// Advent of Code Day 1 Part 1 (12/01/20)
// Written in Rust!
// Author: Logan D.G. Smith
// Reminded me heavily of the "Sock Merchant" problem.
// We known that for some number, there exists a complement
// in the list that will equal 2020. Use a hashmap to keep track
// of what we've seen with O(1) lookup and insertion
use advent_of_rust_2020::read_lines;
use std::collections::HashMap;

fn main() {
    // Static Variables
    let target_sum = 2020;  // Our number pair should add to this

    // Data Structures
    let mut found_values = HashMap::new();  // Determine if we've seen the value' complement

    // Check if file exists
    if let Ok(lines) = read_lines("inputs/day1input.txt") {
        // Read line by line
        for line in lines {
            if let Ok(entry) = line {
                match entry.parse::<i32>() {
                    // Value is an i32 number
                    Ok(num) => {
                        // Evaluate number
                        let complement = target_sum - num;

                        // Found the number and its complement
                        if found_values.contains_key(&complement) {
                            println!("Day One Part One:");
                            println!("Found Numbers: {}, {}", num, complement);
                            println!("Product of Numbers: {}\n", num * complement);
                        }
                        // Add num to the hashmap
                        else {
                            found_values.insert(num, complement);
                        }

                    },
                    // Value is not an i32 number
                    Err(_) => {
                        println!("Failed to parse!"); // Entry is not an i32
                        return;
                    }
                }
            }
        }
    }
    else {
        println!("Couldn't open file!"); // File not found
    }
}
