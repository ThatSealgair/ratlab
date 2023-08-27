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

fn token_position_in_vector(line: &Vec<TokenType>, start_pos: usize, token_type: TokenType) -> Result<usize, bool> {
    let mut index = start_pos + 1;
    for token in line.iter().skip(start_pos) {
        if *token == token_type {
            return Ok(index)
        }
        else {
            index += 1;
        }
    }

    return Err(false)
}

fn get_token_slice(line: &Vec<TokenType>, start_pos: usize, end_pos: usize) -> TokenType {
    let mut identifier_string: String = String::new();
    let mut index = start_pos;

    for token in line.iter().skip(start_pos - 1) {
        
        index += 1;
        if index == end_pos {
            break;
        }
    }

    println!("{}", identifier_string);

    return TokenType::Identifier(identifier_string)
}

fn primitive_start(line: Vec<TokenType>) -> Vec<TokenType> {
    let line_length: usize = line.len() - 1;
    let mut combined_tokens: Vec<TokenType> = Vec::new();

    if !has_token(&line, line_length, TokenType::Syntax(Syntax::SemiColon)) {
        println!("No semicolon");
    }

    let mut position = 0;
    let mut quote_pos = 0;
    let mut d_quote_pos = 0;
    let mut string_type = false;
    let mut char_type = false;
    let mut gets_assigned = false;

    string_type = has_token(&line, position, TokenType::PrimitiveType(PrimitiveType::STRING));
    char_type = has_token(&line, position, TokenType::PrimitiveType(PrimitiveType::CHAR));
    combined_tokens.push(line.index(position).clone());
    position += 1;

    while position < line_length {
        match *line.index(position) {
            TokenType::Syntax(_) => {
                match *line.index(position) {
                    TokenType::Syntax(Syntax::Space) => {
                        combined_tokens.push(line.index(position).clone());
                    },
                    TokenType::Syntax(Syntax::Equals) => {
                        gets_assigned = true;
                        combined_tokens.push(line.index(position).clone());
                    },
                    TokenType::Syntax(Syntax::SemiColon) => {
                        panic!("Semi Colon Invalid syntax!");
                    },
                    TokenType::Syntax(Syntax::Quote) => {
                        if !char_type {
                            panic!("Invalid syntax!");
                        }
                        else {
                            if quote_pos != 0 {
                                panic!("Invalid syntax!");
                            }
                            else {
                                match token_position_in_vector(&line, position, TokenType::Syntax(Syntax::Quote)) {
                                    Ok(pos) => quote_pos = pos,
                                    Err(_) => panic!("Quote syntax error!"),
                                }
                            }
                            combined_tokens.push(get_token_slice(&line, position, quote_pos).clone());
                            position = position + quote_pos;
                        }
                    },
                    TokenType::Syntax(Syntax::DoubleQuote) => {
                        if !string_type {
                            panic!("Invalid syntax!");
                        }
                        else {
                            if d_quote_pos != 0 {
                                panic!("Invalid syntax!");
                            }
                            else {
                                match token_position_in_vector(&line, position, TokenType::Syntax(Syntax::DoubleQuote)) {
                                        Ok(pos) => d_quote_pos = pos,
                                        Err(_) => panic!("Double quote syntax error!"),
                                    }
                            }
                            combined_tokens.push(get_token_slice(&line, position, quote_pos).clone());
                            position = position + d_quote_pos;
                        }
                    }
                    _ => {
                        panic!("Invalid Syntax!");
                    },
                }
            },
            TokenType::Identifier(_) => {
                combined_tokens.push(line.index(position).clone());
            },
            TokenType::NewLine => {
                println!("Remove newlines");
            },
            _ => {
                println!("Invalid token at index {}", position);
                panic!("Invalid token!");
            },
        }
        position += 1;
    }

    if !gets_assigned {
        panic!("Invalid syntax!");
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
