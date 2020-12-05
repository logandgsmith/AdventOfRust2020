use advent_of_rust_2020::read_lines;

fn main() {
    if let Ok(lines) = read_lines("inputs/day5input.txt") {
        let mut highest_sid = 0;
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
                println!("{}: row {}, column {}, seat ID {}.", entry, passenger_row, passenger_col, passenger_sid);
                if passenger_sid > highest_sid { highest_sid = passenger_sid; }
            }
        }

        // Print Results
        println!("\n~~ Day 5 Part 1 ~~");
        println!("Highest Seat ID: {}", highest_sid);
    }
}