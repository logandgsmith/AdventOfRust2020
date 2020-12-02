// Advent of Code Day 1 Part 2 (12/01/20)
// Written in Rust!
// Author: Logan D.G. Smith
// Variation of the 3 sums problem, so we can no longer
// assume that some complement exists.
// Used a brute force method of finding the 3 numbers.
use advent_of_rust_2020::read_lines;

fn main() {
    // Static Variables
    let target_sum = 2020; // Our number pair should add to this

    // Data Structures
    let mut all_values = Vec::new();

    // Check if file exists
    if let Ok(lines) = read_lines("inputs/day1input.txt") {
        // Read line by line
        for line in lines {
            if let Ok(entry) = line {
                match entry.parse::<i32>() {
                    // Value is an i32 number
                    Ok(num) => all_values.push(num),
                    // Value is not an i32 number
                    Err(_) => {
                        println!("Failed to parse!"); // Entry is not an i32
                        return;
                    }
                }
            }
        }

        // Part 2 Evaluations
        all_values.sort();
        for value in &all_values {
            for value2 in &all_values {
                for value3 in &all_values {
                    if value + value2 + value3 == target_sum {
                        println!("Part Two:");
                        println!("Numbers: {}, {}, {}", value, value2, value3);
                        println!("Product: {}", value * value2 * value3);
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