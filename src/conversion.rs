/* Handles the conversion of validated tokens to compilation-ready rust code.
 */

use crate::header::*;

enum TokenClump {
    Single(TokenType),
    TypeClump(Typing),
    Clump(Vec<TokenClump>),
}

struct Typing {
    prim: PrimitiveType,
    name: String,
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
    let mut needed_wrap = false;
    match clump {
        TokenClump::Single(var) => {
            let mut string = String::new();
            if needs_wrap {string.push(syntax::LEFT_BRACKET); needed_wrap = true; needs_wrap = false;};
            string.push_str(match var {
                TokenType::PrimitiveType(prim) => prim_to_string(prim).as_str(),
                TokenType::Conditional(cond) => "",
                TokenType::Syntax(syn) => "",
                TokenType::Identifier(ident) => "",
                TokenType::Statements(stm) => "",
                TokenType::ArithmeticOperator(ar_op) => "",
            });
            if needed_wrap {string.push(syntax::RIGHT_BRACKET); needed_wrap = false;};
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
        },
        TokenClump::TypeClump(var) => {
            String::new()
        }
    }
}

/* Turns primitive type token into string.
 */
fn prim_to_string(prim: PrimitiveType) -> String {
    match prim {
        PrimitiveType::BOOL => PrimitiveType::BOOL.to_rust(),
        PrimitiveType::CHAR => PrimitiveType::CHAR.to_rust(),
        PrimitiveType::STRING => PrimitiveType::STRING.to_rust(),
        PrimitiveType::INT8 => PrimitiveType::INT8.to_rust(),
        PrimitiveType::UINT8 => PrimitiveType::UINT8.to_rust(),
        PrimitiveType::INT16 => PrimitiveType::INT16.to_rust(),
        PrimitiveType::UINT16 => PrimitiveType::UINT16.to_rust(),
        PrimitiveType::INT32 => PrimitiveType::INT32.to_rust(),
        PrimitiveType::UINT32 => PrimitiveType::UINT32.to_rust(),
        PrimitiveType::INT64 => PrimitiveType::INT64.to_rust(),
        PrimitiveType::UINT64 => PrimitiveType::UINT64.to_rust(),
        PrimitiveType::SINGLE => PrimitiveType::SINGLE.to_rust(),
        PrimitiveType::DOUBLE => PrimitiveType::DOUBLE.to_rust(),
    }.to_string()
}
