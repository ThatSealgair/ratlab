use std::env;

use std::fs::File;
use std::io::prelude::*;
use ratlib::input::*;
use ratlib::tokeniser::*;
use ratlib::validation::*;
use ratlib::conversion::*;
use ratlib::header::*;

fn write_to_file(strings: Vec<String>) {
    let mut file = File::create("rat.rs").expect("H");

    let header = vec![
        String::from("fn main() {"),
    ];

    for h in header {
        file.write_all(h.as_bytes()).expect("H");
        file.write_all(b"\n").expect("H");
    }

    for s in strings {
        file.write_all(s.as_bytes()).expect("H");
        file.write_all(b"\n").expect("H");
    }

    let footer = vec![
        String::from("}"),
    ];

    for f in footer {
        file.write_all(f.as_bytes()).expect("H");
        file.write_all(b"\n").expect("H");
    }
    
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Incorrect usage, please try 'ratlab [file]'.");
    }
    let lines: Vec<String> = ratlab_input(args[1].as_str());

    let tokens: Vec<Vec<TokenType>> = tokenize(lines);
    let validated: Vec<Vec<TokenType>> = ratlab_validation(tokens);
    let outputs: Vec<String> = ratlab_conversion(validated);
   
    write_to_file(outputs);
}
