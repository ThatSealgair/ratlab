/* Handles the conversion of validated tokens to compilation-ready rust code.
 */

use crate::header::*;

enum TokenClump {
    Single(TokenType),
    Clump(Vec<TokenClump>),
}

/* Takes in the validated tokens and outputs the converted lines.
 */
pub fn ratlab_conversion(tokens: Vec<TokenType>) -> Vec<String> {
    Vec::new()
}

/* Iterates through a line, breaking it into clumps
 */
fn token_clump(tokens: &Vec<TokenType>, index: usize) -> (Vec<TokenClump>, usize) {
    let clumps: Vec<TokenClump> = Vec::new();
    let index: u64 = 0;
    for token in tokens.iter() {
        match *token => {
            TokenType::Syntax(Syntax::LeftBracket) => token_clump(tokens, index + 1),
            TokenType::Syntax(Syntax::RightBracket) => return (clumps, index + 1),
            (_) => clumps.push(tokens[i]),
        };
        index += 1;
    }
    (Vec::new(), 0)
}
