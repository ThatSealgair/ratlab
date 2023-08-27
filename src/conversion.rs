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
    let mut lines: Vec<String> = Vec::new();
    for line in tokens {
        let mut line_string: String = String::new();
        let (clump, _) = token_clump(&line, 0usize);
        let is_dec: bool = match clump {
            TokenClump::Clump(ref var) => {
                match var.first() {
                    Some(TokenClump::TypeClump(_)) => true,
                    _ => false,
                }
            },
            _ => false,
        };
        if is_dec {
            line_string.push_str("let mut ");
        }
        let text: String = clump_to_string(clump);
        let line_len: usize = text.len();
        line_string.push_str(&text[1..line_len-1]);
        if line_len > 2 {
            line_string.push(';');
        }
        lines.push(line_string);
    }
    lines
}

/* Iterates through a line, breaking it into clumps
 */
fn token_clump(tokens: &Vec<TokenType>, index: usize) -> (TokenClump, usize) {
    let mut clumps: Vec<TokenClump> = Vec::new();
    let mut i: usize = index;
    let mut iterator = tokens[index..].iter();
    loop {
        let token = match iterator.next() {
            Some(var) => var,
            None => break,
        };
        let (clump_vec, _) = match token {
            TokenType::Syntax(Syntax::LeftBracket) => token_clump(tokens, i + 1usize),
            TokenType::Syntax(Syntax::RightBracket) => return (TokenClump::Clump(clumps), i + 1usize),
            TokenType::PrimitiveType(var) => {
                let mut typing: Typing = Typing {prim: var.clone(), name: "".to_string()};
                iterator.next();
                typing.name = match iterator.next() {
                    Some(TokenType::Identifier(val)) => val.to_string(),
                    _ => panic!("Not validated right."),
                };
                i += 2;
                (TokenClump::TypeClump(typing), i)
            },
            var => (TokenClump::Single(var.clone()), i),
        };
        let clump_size = match &clump_vec {
            TokenClump::Single(_) => 1usize,
            TokenClump::Clump(var) => var.len(),
            TokenClump::TypeClump(_) => 1usize,
        };
        clumps.push(clump_vec);
        i += clump_size;
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
                TokenType::PrimitiveType(prim) => prim.to_rust().to_string(),
                TokenType::Conditional(cond) => cond.to_rust().to_string(),
                TokenType::Syntax(syn) => syn.to_rust(),
                TokenType::Identifier(ident) => ident,
                TokenType::Statements(stm) => stm.to_rust().to_string(),
                TokenType::ArithmeticOperator(ar_op) => ar_op.to_rust().to_string(),
            }.as_str());
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
            let mut string = String::new();
            string.push_str(var.name.as_str());
            string.push_str(": ");
            string.push_str(var.prim.to_rust());
            string
        }
    }
}

/* Turns primitive type token into string. Hopefully unneccesary.
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
