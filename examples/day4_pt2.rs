// Advent of Code Day 3 Part 1 (12/03/20)
// Written in Rust!
// Author: Logan D.G. Smith
use regex::RegexSet;
use advent_of_rust_2020::read_lines;

fn main() {
    let mut valid_passports: i32 = 0;

    // Possible codes on a passport, cid is optional
    let passport_codes = RegexSet::new(&[
        r"byr:(19[2-9][0-9]|200[0-2])",
        r"iyr:(20(1[0-9]|20))",
        r"eyr:(20(2[0-9]|30))",
        r"hgt:((1([5-8][0-9]|9[0-3])cm)|((59|6[0-9]|7[0-6])in))",
        r"hcl:(#[0-9a-f]{6})",
        r"ecl:(amb|blu|brn|gry|grn|hzl|oth)",
        r"pid:([0-9]{9})",
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
                        valid_passports += 1;
                        println!("Valid:   {} {}", complete_line, valid_passports);
                    }
                    else {
                        //println!("Invalid: {}", complete_line);
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
    println!("Number of valid passports: {}", valid_passports);
}