use ratlib::input::*;
use ratlib::header::*;

fn main() {
    let lines: Vec<String> = ratlab_input("file");
    
    println!("Type name for characters is {}.", primatives::CHAR);
    println!("----------");

    for line in lines.iter() {
        for string in line.lines() {
            print!("{}", string);
        }
        //println!("");
    }
}
