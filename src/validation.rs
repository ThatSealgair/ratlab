/* Token validation for ratlab
 */ 

// Library imports
use crate::header::*;

/* Main function for token validation.
 */
fn ratlab_validation(tokens: Vec<TokenType>) {
    for token in tokens.iter() {
        match &token {
            TokenType::NewLine => {
                println!("New line!");
            },
            TokenType::Syntax(Syntax::Space) => {
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
            TokenType::ArithmeticOperator(_) => {
                println!("Arithmetic!");
            }
            TokenType::Conditional(_) => {
                println!("Conditional!");
            },
            _ => {
                println!("To add!");
            }
        }
    }
}
