/* Token validation for ratlab
 */ 

use std::ops::Index;

// Library imports
use crate::header::*;

/* Main function for token validation.
 */
fn ratlab_validation(tokens: Vec<Vec<TokenType>>) {
    let mut current_line: Vec<TokenType> = Vec::new();
    let mut valid_lines: Vec<Vec<TokenType>> = Vec::new();

    for line in tokens.iter() {
        match line.index(0) {
        //match &token {
            TokenType::NewLine => {
                println!("New line!");
            },
            TokenType::Syntax(_) => {
                println!("Space!");
            },
            TokenType::PrimitiveType(_) => {
                println!("PrimitiveType")
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
}
