use std::env;

use ratlib::input::*;
use ratlib::tokeniser::*;
use ratlib::validation::*;
use ratlib::conversion::*;
use ratlib::header::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Incorrect usage, please try 'ratlab [file]'.");
    }
    let lines: Vec<String> = ratlab_input(args[1].as_str());

    let mut tokens: Vec<Vec<TokenType>> = tokenize(lines);
    ratlab_validation(tokens); 

    //println!("Contents of {} are:", args[1].as_str());
    //println!("----------");
    
    //let mut index = 0;
    //for line in lines.iter() {
    //    println!("{index} | {line}");
    //    index += 1;
    //}
}
