/// tokeniser for ratlab

use crate::header::*;

fn tokenize_type(data: &str) -> TokenType {
    match data {
        CHAR => return TokenType::Char,
        BOOL => return TokenType::Bool,
        INT_8 => return TokenType::Int8,
        UINT_8 => return TokenType::Uint8,
        INT_16 => return TokenType::Int16,
        UINT_16 => return TokenType::Uint16,
        INT_32 => return TokenType::Int32,
        UINT_32 => return TokenType::Uint32,
        INT_64 => return TokenType::Int64,
        UINT_64 => return TokenType::Uint64,
        FLOAT => return TokenType::Single,
        DOUBLE => return TokenType::Double,
    }
}


fn tokenize_string(data: &str) -> TokenType {
    return TokenType::Identifier(data.to_string())
}


fn tokenize(data: Vec<String>) -> Vec<TokenType> {
    let mut tokens: Vec<TokenType> = Vec::new();

    for line in data.iter() {
        let mut is_token = false;
        let mut token = Vec::new();

        for ch in line.chars() {
            let tab = Syntax::Tab.to_char();
            let space = Syntax::Space.to_char();
            let semi_colon = Syntax::SemiColon.to_char();
            let colon = Syntax::Colon.to_char();
            let peroid = Syntax::Peroid.to_char();
            let comma = Syntax::Comma.to_char();
            let equals = Syntax::Equals.to_char();
            let left_brace = Syntax::LeftBrace.to_char();
            let right_brace = Syntax::RightBrace.to_char();
            let left_bracket = Syntax::RightBracket.to_char();
            let right_bracket = Syntax::RightBracket.to_char();
            match ch {
                tab => {
                    if is_token {
                        tokens.push(
                            tokenize_string(&token
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    let symbol = Syntax::Tab;
                    tokens.push(TokenType::Syntax(symbol));
                },
                space => {
                    if is_token {
                        tokens.push(
                            tokenize_string(&token
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    tokens.push(TokenType::Syntax(Syntax::Space));
                },
                semi_colon => {
                    if is_token {
                        tokens.push(
                            tokenize_string(&token
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    tokens.push(TokenType::Syntax(Syntax::SemiColon));
                },
                colon => {
                    if is_token {
                        tokens.push(
                            tokenize_string(&token
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    tokens.push(TokenType::Syntax(Syntax::Colon));
                },
                peroid => {
                    if is_token {
                        tokens.push(
                            tokenize_string(&token
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    tokens.push(TokenType::Syntax(Syntax::Peroid));
                },
                comma => {
                    if is_token {
                        tokens.push(
                            tokenize_string(&token
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    tokens.push(TokenType::Syntax(Syntax::Comma));
                },
                equals => {
                    if is_token {
                        tokens.push(
                            tokenize_string(&token
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    tokens.push(TokenType::Syntax(Syntax::Equals));
                },
                left_brace => {
                    if is_token {
                        tokens.push(
                            tokenize_string(&token
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    tokens.push(TokenType::Syntax(Syntax::LeftBrace));
                },
                right_brace => {
                    if is_token {
                        tokens.push(
                            tokenize_string(&token
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    tokens.push(TokenType::Syntax(Syntax::RightBrace));
                },
                left_bracket => {
                    if is_token {
                        tokens.push(
                            tokenize_string(&token
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    tokens.push(TokenType::Syntax(Syntax::LeftBracket));
                },
                right_bracket => {
                    if is_token {
                        tokens.push(
                            tokenize_string(&token
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    tokens.push(TokenType::Syntax(Syntax::RightBracket));
                },
                _ => {
                    is_token = true;
                    token.push(ch);
                },
            }
        }
    }

    return tokens
}
