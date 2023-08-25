mod input;

use crate::input::*;

fn main() {
    let lines: Vec<String> = ratlab_input("file");

    for line in lines.iter() {
        println!("{}", line);
    }
}
