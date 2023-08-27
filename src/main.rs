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

    let tokens: Vec<Vec<TokenType>> = tokenize(lines);
    let validated: Vec<Vec<TokenType>> = ratlab_validation(tokens);
    let outputs: Vec<String> = ratlab_conversion(validated);
    
    let mut i: u16 = 0;
    for line in outputs {
        i += 1;
        println!("{} | {}", i, line);
    }
}
