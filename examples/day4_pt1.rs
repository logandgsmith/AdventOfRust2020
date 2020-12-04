// Advent of Code Day 3 Part 1 (12/03/20)
// Written in Rust!
// Author: Logan D.G. Smith
use regex::RegexSet;
use advent_of_rust_2020::read_lines;

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
                        println!("Valid:   {}", complete_line);
                        valid_passwords += 1;
                    }
                    else {
                        println!("Invalid: {}", complete_line);
                    }
                    complete_line = "".to_string();
                }
                else {
                   complete_line = format!("{} {}", complete_line, entry);
                }
            }
        }
    }

    println!("\nDay 4 Part 1");
    println!("Number of valid passports: {}", valid_passwords);
}