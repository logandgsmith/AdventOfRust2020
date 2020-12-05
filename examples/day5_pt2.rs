use advent_of_rust_2020::*;

fn main() {
    if let Ok(lines) = read_lines("inputs/day5input.txt") {
        let mut seats = Vec::new();
        // Read line by line
        for line in lines {
            if let Ok(entry) = line {
                // Find row
                let row_string = entry[..7].chars().rev().collect::<String>();
                let mut passenger_row = 0;
                for (index, row) in row_string.chars().enumerate() {
                    if row == 'B' {
                        passenger_row += 2_i32.pow(index as u32);
                    }
                }

                // Find col
                let col_string = entry[7..].chars().rev().collect::<String>();
                let mut passenger_col = 0;
                for (index, col) in col_string.chars().enumerate() {
                    if col == 'R' {
                        passenger_col += 2_i32.pow(index as u32);
                    }
                }

                // Find Seat ID
                let passenger_sid = passenger_row * 8 + passenger_col;
                seats.push(passenger_sid);
            }
        }
        
        seats.sort();
        let mut unfilled_seat = 0;
        for i in 1..seats.len() - 1 {
            if seats[i] - 1 != seats[i - 1] && seats[i] - 2 == seats[i - 1] {
                unfilled_seat = seats[i] - 1;
                valid_line(&format!("Your seat is {}", unfilled_seat));   
            } 
            invalid_line(&format!("Seat filled at {}", &seats[i]));

        }

        // Print Results
        println!("\n~~ Day 5 Part 2 ~~");
        valid_line(&format!("Your seat is {}", unfilled_seat));
    }
}