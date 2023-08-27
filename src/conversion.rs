/* Handles the conversion of validated tokens to compilation-ready rust code.
 */

use crate::header::*;

enum TokenClump {
    Single(TokenType),
    Clump(Vec<TokenClump>),
}

/* Takes in the validated tokens and outputs the converted lines.
 */
pub fn ratlab_conversion(tokens: Vec<Vec<TokenType>>) -> Vec<String> {
    let lines: Vec<String> = Vec::new();
    for line in tokens {
        let (clump, _) = token_clump(&line, 0usize);
        
    }
    lines
}

/* Iterates through a line, breaking it into clumps
 */
fn token_clump(tokens: &Vec<TokenType>, index: usize) -> (TokenClump, usize) {
    let clumps: Vec<TokenClump> = Vec::new();
    let mut i: usize = index;
    for token in tokens[index..].iter() {
        let (clump_vec, _) = match *token {
            TokenType::Syntax(Syntax::LeftBracket) => token_clump(tokens, i + 1usize),
            TokenType::Syntax(Syntax::RightBracket) => return (TokenClump::Clump(clumps), i + 1usize),
            var => (TokenClump::Single(var), index),
        };
        i += 1;
    }
    (TokenClump::Clump(clumps), i)
}

/* Converts token clumps to rust strings.
 */
fn clump_to_string(clump: TokenClump) -> String {
    let mut needs_wrap = false;
    match clump {
        TokenClump::Single(var) => {
            let mut string = String::new();
            if needs_wrap {string.push(syntax::LEFT_BRACKET)};
            string.push_str(match var {
                TokenType::PrimitiveType(type) => "".to_string(),
                TokenType::Conditional(type) => "".to_string(),
                TokenType::Syntax(type) => "".to_string(),
                TokenType::Identifier(type) => "".to_string(),
                TokenType::Statements(type) => "".to_string(),
                TokenType::ArithmeticOperator(type) => "".to_string(),
            }.as_str());
            if needs_wrap {string.push(syntax::RIGHT_BRACKET)};
            string
        },
        TokenClump::Clump(vars) => {
            let mut string = String::new();
            string.push(syntax::LEFT_BRACKET);
            for c in vars {
                string.push_str(clump_to_string(c).as_str());
            }
            string.push(syntax::RIGHT_BRACKET);
            string
        }
    }
}

