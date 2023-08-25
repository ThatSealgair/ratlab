mod input;
mod header;

use crate::input::*;
use crate::header::*;

fn main() {
    let lines: Vec<String> = ratlab_input("file");
    
    println!("Type name for characters is {}.", types::CHAR);
    println!("----------");

    for line in lines.iter() {
        println!("{}", line);
    }
}
