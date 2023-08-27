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

fn get_identifier_slice(line: &Vec<TokenType>, start_pos: usize, end_pos: usize) -> TokenType {
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
                            panic!("Invalid syntax! Not type char!");
                        }
                        else {
                            if quote_pos != 0 {
                                panic!("Invalid syntax! Quotes, set!");
                            }
                            else {
                                match token_position_in_vector(&line, position, TokenType::Syntax(Syntax::Quote)) {
                                    Ok(pos) => quote_pos = pos,
                                    Err(_) => panic!("Quote syntax error!"),
                                }
                            }
                            combined_tokens.push(get_identifier_slice(&line, position, quote_pos).clone());
                            position = position + quote_pos;
                        }
                    },
                    TokenType::Syntax(Syntax::DoubleQuote) => {
                        if !string_type {
                            panic!("Invalid syntax! Not string type!");
                        }
                        else {
                            if d_quote_pos != 0 {
                                panic!("Invalid syntax! Double quotes set!");
                            }
                            else {
                                match token_position_in_vector(&line, position, TokenType::Syntax(Syntax::DoubleQuote)) {
                                        Ok(pos) => d_quote_pos = pos,
                                        Err(_) => panic!("Double quote syntax error!"),
                                    }
                            }
                            combined_tokens.push(get_identifier_slice(&line, position, quote_pos).clone());
                            position = position + d_quote_pos;
                        }
                    }
                    _ => {
                        panic!("Invalid Syntax! Cannot set variable!");
                    },
                }
            },
            TokenType::Identifier(_) => {
                combined_tokens.push(line.index(position).clone());
            },
            _ => {
                println!("Invalid token at index {}", position);
                panic!("Invalid token!");
            },
        }
        position += 1;
    }

    if !gets_assigned {
        panic!("Invalid syntax! Variable unset!");
    }

    return combined_token
}


fn identifier_start(line: Vec<TokenType>) -> Vec<TokenType> {
    let line_length: usize = line.len() - 1;
    let mut combined_tokens: Vec<TokenType> = Vec::new();

    if !has_token(&line, line_length, TokenType::Syntax(Syntax::SemiColon)) {
        panic!("No semicolon!");
    }
    
    if line_length != 4 {
        panic!("Invalid function call length of {}", line_length);
    }

    let mut position = 0;
    let mut brace_pos = 0;
    let mut gets_assigned = false;

    combined_tokens.push(line.index(position).clone());
    position += 1;

    while position < line_length {
        println!("position {}", position);
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
                    TokenType::Syntax(Syntax::LeftBrace) => {
                        match token_position_in_vector(&line, position, TokenType::Syntax(Syntax::RightBrace)) {
                            Ok(pos) => brace_pos = pos,
                            Err(_) => panic!("Invalid braces!"),
                        }
                        combined_tokens.push(get_identifier_slice(&line, position, brace_pos).clone());
                        position = position + brace_pos;
                    },
                    _ => {
                        panic!("Invalid Syntax for function call!");
                    },
                }
            },
            TokenType::Identifier(_) => {
                combined_tokens.push(line.index(position).clone());
            },
            _ => {
                println!("Invalid token at index {}", position);
                panic!("Invalid token!");
            },
        }
        position += 1;
    }

    return combined_tokens
}



/* Main function for token validation.
 */
pub fn ratlab_validation(tokens: Vec<Vec<TokenType>>) -> Vec<Vec<TokenType>> {
    let mut current_line: Vec<TokenType> = Vec::new();
    let mut valid_lines: Vec<Vec<TokenType>> = Vec::new();

    for line in tokens.iter() {
        if let Some(token) = line.first() {
            match token {
                TokenType::Syntax(_) => {
                    println!("Syntax!");
                },
                TokenType::PrimitiveType(_) => {
                    println!("Primitive!");
                    current_line = primitive_start(line.clone());
                    
                },
                TokenType::Identifier(_) => {
                    println!("Identifier!");
                    current_line = identifier_start(line.clone());
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
        println!("I am appending a valid line!");

        valid_lines.push(current_line.clone());
    }
}
