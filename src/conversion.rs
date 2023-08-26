/* Handles the conversion of validated tokens to compilation-ready rust code.
 */

use crate::header;

enum TokenClump {
    Single(TokenType),
    Clump(Vec<TokenClump>),
}

/* Takes in the validated tokens and outputs the converted lines.
 */
pub fn ratlab_conversion(tokens: Vec<TokenType>) -> Vec<String> {

}

/* Iterates through a line, breaking it into clumps
 */
fn token_clump(tokens: &Vec<TokenType>, index: usize) -> (Vec<TokenClump>, usize) {
    let clumps: Vec<TokenClump> = Vec::new();
    for i in 0..tokens.len() {
        match tokens[i] => {
            TokenType::Syntax(Syntax::LeftBracket) => token_clump(tokens, index + 1);
            TokenType::Syntax(Syntax::RightBracket) => return (clumps, index + 1);
            (_) => clumps.push(tokens[i]);
        };
    }
}
