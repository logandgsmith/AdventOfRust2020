// Advent of Code Day 5 Part 2 (12/05/20)
// Written in Rust!
// Author: Logan D.G. Smith
// This part of the exercise made me wish I had gone with the binary tree
// in the first part. In the end, I'm still happy with how it turned out
// and I'd be interested in rewriting some of the functions to be more
// idiomatic.
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
        let mut seats = Vec::new();
        // Read line by line
        for line in lines {
            if let Ok(entry) = line {
                // Find Seat ID
                let passenger_sid = translate_ticket(&entry);
                seats.push(passenger_sid);
            }
        }
        
        // Sort the list by Seat ID
        seats.sort();

        let mut unfilled_seat = 0;
        for i in 1..seats.len() - 1 {
            // Check if there's a seat missing.
            if seats[i] - 1 != seats[i - 1] && seats[i] - 2 == seats[i - 1] {
                unfilled_seat = seats[i] - 1;
                valid_line(&format!("Your seat is {}", unfilled_seat));   
            }
            // Print filled seat
            invalid_line(&format!("Seat filled at {}", &seats[i]));
        }

        // Print Results
        println!("\n~~ Day 5 Part 2 ~~");
        valid_line(&format!("Your seat is {}", unfilled_seat));
    }
}