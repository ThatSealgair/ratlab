use ratlab::input::*;
use ratlab::header::*;

fn main() {
    let lines: Vec<String> = ratlab_input("file");
    
    println!("Type name for characters is {}.", types::CHAR);
    println!("----------");

    for line in lines.iter() {
        for string in line.lines() {
            print!("{}", string);
        }
        //println!("");
    }
}
