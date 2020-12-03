// Advent of Code Day 3 Part 2 (12/03/20)
// Written in Rust!
// Author: Logan D.G. Smith
// I used this exercise to learn more about objects in Rust.
// While it definitely could be more optimized, I learned a lot
// about how NOT to use/implement objects along the way. Also
// surprising was how to access the paths held in a vector!
use advent_of_rust_2020::read_lines;

struct Slope {
    // Holds all of the information about our slope
    right: usize,
    down: usize,
    column: usize,
    trees: usize,
}

impl Slope {
    // Implement the Slope struct 
    fn new(right: usize, down: usize) -> Slope {
        Slope {
            right,
            down,
            column: 0,
            trees: 0,
        }
    } 
}

fn main() { 
    let mut paths = Vec::new();

    // Add slopes to the possible paths
    paths.push(Slope::new(1, 1));
    paths.push(Slope::new(3, 1));
    paths.push(Slope::new(5, 1));
    paths.push(Slope::new(7, 1));
    paths.push(Slope::new(1, 2));

    // Check if file exists
    if let Ok(lines) = read_lines("inputs/day3input.txt") {
        let mut line_number = 0;
        // Read line by line
        for line in lines {
            if let Ok(entry) = line {
                let length = entry.len();
                // Skip empty strings
                if length == 0 { continue; }

                // Checking each path for trees
                for mut path in &mut paths {
                    // Skip if we need to go down more
                    if line_number % path.down != 0 { continue; }

                    // Parse for trees
                    if entry.chars().nth(path.column).unwrap() == '#' {
                        path.trees += 1;
                    }

                    // Update the column if necessary
                    path.column = (path.column + path.right) % length; 
                }         
            }
            else {
                println!("Failed to parse");
            }
            line_number += 1; // Keep track of line number for down slope
        }
    }

    // Output
    println!("Day 3 Part 2\n");
    let mut trees_product = 1;
    for path in paths {
        println!("Path: ({}, {}) has {} trees.", path.right, path.down, path.trees);
        trees_product *= path.trees;
    }
    println!("Product of trees: {}", trees_product);
}