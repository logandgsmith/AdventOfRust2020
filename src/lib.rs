// Helper functions that are useful on multiple exercises

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// See https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// Returns Iterator to the Reader of the lines of the File
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {                     // AsRef dereferences to filename to a Path object
    let file = File::open(filename)?;       // The `?` operator checks if result is Ok
    Ok(io::BufReader::new(file).lines())    // No `;` means the function will return this value
}

// Helper function to print out a green line and declare it valid
pub fn valid_line(line: &str) {
    println!("\u{001B}[32mValid:   {}\u{001B}[0m", line);
}

// Helper function to print out a red line and declare it valid
pub fn invalid_line(line: &str) {
    println!("\u{001B}[31mInvalid: {}\u{001B}[0m", line);
}