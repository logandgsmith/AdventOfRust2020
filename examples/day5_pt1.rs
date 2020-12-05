// Advent of Code Day 5 Part 1 (12/05/20)
// Written in Rust!
// Author: Logan D.G. Smith
// When I was first reading the exercise, I thought that I would end up
// implementing a binary tree, but for this first exercise, there wasn't
// a pressing need to.
use advent_of_rust_2020::*;

// Calculate the binary value of a string
fn translate_binary_string(substring: &str) -> i32 {
    let mut substring_value: i32 = 0;

    // Break the substring into chars and enumerate (get their index)
    for (index, entry) in substring.chars().enumerate() {
        if entry == 'B' || entry == 'R' {
            substring_value += 2_i32.pow(index as u32);
        }
    }

    substring_value
}

// Calculate the Seat ID of a ticket string
fn translate_ticket(ticket: &str) -> i32 {
    // Tickets must have a length of 10 characters
    assert!(ticket.len() == 10);

    let mut ticket_value = 0;

    // Translate the row 
    ticket_value += translate_binary_string(
        &ticket[..7]
        .chars()
        .rev()
        .collect::<String>()
    ) * 8;

    // Translate the col
    ticket_value += 
    translate_binary_string(
        &ticket[7..]
        .chars()
        .rev()
        .collect::<String>()
    );

    ticket_value
}

fn main() {
    if let Ok(lines) = read_lines("inputs/day5input.txt") {
        let mut highest_sid = 0;
        // Read line by line
        for line in lines {
            if let Ok(entry) = line {
                // Find Seat ID
                let passenger_sid = translate_ticket(&entry);
                if passenger_sid > highest_sid { highest_sid = passenger_sid; }
            }
        }

        // Print Results
        println!("\n~~ Day 5 Part 1 ~~");
        valid_line(&format!("Highest Seat ID: {}", highest_sid));
    }
}