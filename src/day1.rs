// Advent of Code Day 1
// Written in Rust!

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

// See https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// Returns Iterator to the Reader of the lines of the File
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {                     // AsRef dereferences to filename to a Path object
    let file = File::open(filename)?;       // The `?` operator checks if result is Ok
    Ok(io::BufReader::new(file).lines())    // No `;` means the function will return this value
}

fn main() {
    // Static Variables
    let target_sum = 2020;                  // Our number pair should add to this

    // Data Structures
    let mut found_values = HashMap::new();  // Determine if we've seen the value' complement
    let mut all_values = Vec::new();

    // Check if file exists
    if let Ok(lines) = read_lines("./input/day1input.txt") {
        // Read line by line
        for line in lines {
            if let Ok(entry) = line {
                match entry.parse::<i32>() {
                    // Value is an i32 number
                    Ok(num) => {
                        // Evaluate number
                        let complement = target_sum - num;
                        all_values.push(num);

                        // Found the number and its complement
                        if found_values.contains_key(&complement) {
                            println!("Part One:");
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
