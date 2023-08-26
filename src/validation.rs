/* Token validation for ratlab
 */ 

use std::{ops::Index, primitive};

// Library imports
use crate::header::*;

fn has_token(line: &Vec<TokenType>, index: usize, token_type: TokenType) -> bool {
    if *line.index(index) == token_type {
        return true
    }
    else {
        false
    }
}

fn primitive_start(line: Vec<TokenType>) -> Vec<TokenType> {
    let line_length: usize = line.len();

    if !has_token(&line, line_length - 1, TokenType::Syntax(Syntax::SemiColon)) {
        println!("No semicolon");
    }

    let mut position = 0;
    let mut sub_pos_a = 0;
    let mut sub_pos_b = 0;
    let mut string_type = false;
    let mut char_type = false;
    let mut gets_assigned = false;

    string_type = has_token(&line, position, TokenType::PrimitiveType(PrimitiveType::STRING));
    char_type = has_token(&line, position, TokenType::PrimitiveType(PrimitiveType::CHAR));

    position += 1;

    while position < line_length {
        match *line.index(position) {
            TokenType::Syntax(_) => {
                match *line.index(position) {
                    TokenType::Syntax(Syntax::Space) => {
                        println!("Space good!");
                    },
                    TokenType::Syntax(Syntax::Equals) => {
                        println!("Variable assigned!");
                        gets_assigned = true;
                    },
                    TokenType::Syntax(Syntax::SemiColon) => {
                        if (position != line_length) {
                            println!("Too many semi colons!");
                        }
                    },
                    _ => {
                        println!("Invalid syntax token!");
                    },
                }
            },
            TokenType::Identifier(_) => {
                println!("Identifier good!");
            },
            TokenType::NewLine => {
                println!("Remove newlines");
            },
            _ => {
                println!("Invalid token at index {}", position);
            },
        }
        position += 1;
    }

    if !gets_assigned {
        println!("Doesn't get assign");
    }

    return line
}

/* Main function for token validation.
 */
pub fn ratlab_validation(tokens: Vec<Vec<TokenType>>) {
    let mut current_line: Vec<TokenType> = Vec::new();
    let mut valid_lines: Vec<Vec<TokenType>> = Vec::new();

    for line in tokens.iter() {
        if let Some(token) = line.first() {
            match token {
            //match &token {
                TokenType::NewLine => {
                    println!("New line!");
                },
                TokenType::Syntax(_) => {
                    match &token {
                        TokenType::Syntax(Syntax::Tab) => println!("Tab!"),
                        TokenType::Syntax(Syntax::Space) => println!("Space!"),
                        TokenType::Syntax(Syntax::Colon) => println!("Colon!"),
                        TokenType::Syntax(Syntax::Comma) => println!("Comma!"),
                        _ => println!("Other Syntax!"),
                    }
                    println!("Syntax! {}", token.to_string());
                },
                TokenType::PrimitiveType(_) => {
                    current_line = primitive_start(line.clone());
                },
                TokenType::Identifier(_) => {
                    println!("Identifier!");
                },
                TokenType::Statements(_) => {
                    println!("Statement!");
                },
                TokenType::Conditional(_) => {
                    println!("Conditional!");
                },
                _ => {
                    println!("Skip this");
                }
            }
        }

        valid_lines.push(current_line.clone());
    }
}
