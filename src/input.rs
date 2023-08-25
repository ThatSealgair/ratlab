/* ratlab Input Phase
 * ----------
 * Provides functions for checking that a file exists, opening it, and checking 
 * whether it's empty.
 */

// Library imports
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

/* Main function to handle the input process.
 */
pub fn ratlab_input(file_path: &str) -> Vec<String> {
    if !does_file_exist(file_path) {
        panic!("No file exists at '{}'.", file_path);
    }
    file_to_lines(open_file(file_path))
}

/* Simple helper function to check whether a file exists.
 */
fn does_file_exist(file_path: &str) -> bool {
    let path = Path::new(file_path);
    if path.exists() {
        return true;
    }
    false
}

/* Opens up the file given by the path and returns the File struct.
 */
fn open_file(file_path: &str) -> File {
    let path = Path::new(file_path);
    
    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open file '{}', due to {}.", file_path, why),
        Ok(file) => file,
    };

    file
}

fn file_to_lines(file: File) -> Vec<String> {
    let reader = BufReader::new(file);
    let mut lines_vec: Vec<String> = vec![];

    for line in reader.lines() {
        match line {
            Ok(line_val) => lines_vec.push(line_val),
            Err(_) => panic!("File contains invalid line!\n"),
        }
    }

    lines_vec
}
