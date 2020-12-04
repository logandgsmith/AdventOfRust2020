// Advent of Code Day 4 Part 1 (12/04/20)
// Written in Rust!
// Author: Logan D.G. Smith
// Regex matching! It was interesting to see how Rust handled this,
// and from seeing it, I am a big fan of RegexSets now :)
use regex::RegexSet;
use advent_of_rust_2020::*;

fn main() {
    let mut valid_passwords: i32 = 0;

    // Possible codes on a passport, cid is optional
    let passport_codes = RegexSet::new(&[
        r"byr:",
        r"iyr:",
        r"eyr:",
        r"hgt:",
        r"hcl:",
        r"ecl:",
        r"pid:",
        //r"cid:", // optional
    ]).unwrap();

    // Again using the read_lines function from day 1 to read the file
    if let Ok(lines) = read_lines("inputs/day4input.txt") {
        let mut complete_line = "".to_string();
        // Read line by line
        for line in lines {
            if let Ok(entry) = line {
                // Passports are separated by blank lines
                if entry.len() == 0 {
                    let matches: Vec<_> = passport_codes
                                            .matches(&complete_line)
                                            .into_iter()
                                            .collect();
                    
                    // Check if all required fields present
                    if matches.len() == 7 {
                        valid_line(&complete_line);
                        valid_passwords += 1;
                    }
                    // Invalid Number of matches
                    else {
                        invalid_line(&complete_line);
                    }
                    // Reset line to blank string
                    complete_line = "".to_string();
                }
                // Concat lines
                else {
                   complete_line = format!("{} {}", complete_line, entry);
                }
            }
        }
    }

    // Print final result
    println!("\n~~ Day 4 Part 1 ~~");
    println!("Number of valid passports: {}", valid_passwords);
}