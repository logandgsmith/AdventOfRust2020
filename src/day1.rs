// Advent of Code Day 1 Part 1
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
    let target_sum = 2020;                  // Our number pair should add to this

    let mut found_values = HashMap::new();  // Determine if we've seen the value' complement

    // Check if file exists
    if let Ok(lines) = read_lines("./input/day1input.txt") {
        // Read line by line
        for line in lines {
            if let Ok(entry) = line {
                match entry.parse::<i32>() {
                    Ok(num) => {
                        let complement = target_sum - num;

                        // Found the number and its complement
                        if found_values.contains_key(&complement) {
                            println!("Found Numbers: {}, {}", num, complement);
                            println!("Product of Numbers: {}", num * complement);
                        }
                        // Add num to the hashmap
                        else {
                            found_values.insert(num, complement);
                        }

                    },
                    Err(_) => println!("Failed to parse!"), // Entry is not an i32
                }
            }
        }
    }
    else {
        println!("Couldn't open file!"); // File not found
    }
}
