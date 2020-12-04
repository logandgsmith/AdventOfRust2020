// Advent of Code Day 3 Part 2 (12/04/20)
// Written in Rust!
// Author: Logan D.G. Smith
// For this part of the exercise, I ran into the classic regex issue of
// matching the beginning of the string only, meaning if ectra letters
// were appended, we would match regardless.
use regex::RegexSet;
use advent_of_rust_2020::*;

fn main() {
    // Keep track of valid passports spotted
    let mut valid_passports: i32 = 0;

    // Possible codes on a passport, cid is optional
    let passport_codes = RegexSet::new(&[
        r"byr:(19[2-9][0-9]|200[0-2])(\W|$)",                           // 1920-2002
        r"iyr:(20(1[0-9]|20))(\W|$)",                                   // 2010-2020
        r"eyr:(20(2[0-9]|30))(\W|$)",                                   // 2020-2030
        r"hgt:((1([5-8][0-9]|9[0-3])cm)|((59|6[0-9]|7[0-6])in))(\W|$)", // 150cm - 193cm | 59in-76in
        r"hcl:(#[0-9a-f]{6})(\W|$)",                                    // #XXXXXX X = Hex Number
        r"ecl:(amb|blu|brn|gry|grn|hzl|oth)(\W|$)",                     // Eye color from list
        r"pid:([0-9]{9})(\W|$)",                                        // Nine digit pid
        //r"cid:",                                                      // optional
    ]).unwrap();

    // Again using the read_lines function from day 1 to read the file
    if let Ok(lines) = read_lines("inputs/day4input.txt") {
        // Line to concat others to
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
                        valid_line(&complete_line);
                    }
                    // Invalid number of matches
                    else {
                        invalid_line(&complete_line);
                    }
                    // Reset the line to a blank string
                    complete_line = "".to_string();
                }
                else {
                    // Concat the lines
                   complete_line = format!("{} {}", complete_line, entry);
                }
            }
        }
    }

    // Print final result
    println!("\n~~ Day 4 Part 1 ~~");
    println!("Number of valid passports: {}", valid_passports);
}