/// tokeniser for ratlab

use crate::header::*;

fn tokenize_type(data: &str) -> Result<TokenType, bool> {
    match data {
        CHAR => {
            Ok(TokenType::PrimitiveType(PrimitiveType::CHAR))
        },
        BOOL => {
            Ok(TokenType::PrimitiveType(PrimitiveType::BOOL))
        },
        INT_8 => {
            Ok(TokenType::PrimitiveType(PrimitiveType::INT8))
        },
        UINT_8 => {
            Ok(TokenType::PrimitiveType(PrimitiveType::UINT8))
        },
        INT_16 => {
            Ok(TokenType::PrimitiveType(PrimitiveType::INT16))
        },
        UINT_16 => {
            Ok(TokenType::PrimitiveType(PrimitiveType::UINT16))
        },
        INT_32 => {
            Ok(TokenType::PrimitiveType(PrimitiveType::INT32))
        },
        UINT_32 => {
            Ok(TokenType::PrimitiveType(PrimitiveType::UINT32))
        },
        INT_64 => {
            Ok(TokenType::PrimitiveType(PrimitiveType::INT64))
        },
        UINT_64 => {
            Ok(TokenType::PrimitiveType(PrimitiveType::UINT64))
        },
        FLOAT => {
            Ok(TokenType::PrimitiveType(PrimitiveType::SINGLE))
        },
        DOUBLE => {
            Ok(TokenType::PrimitiveType(PrimitiveType::DOUBLE))
        },
        _ => {
            Err(false)
        },
    }
}

fn statement_token(data: &str) -> Result<TokenType, bool> {
    let while_str = Statements::While.to_string();
    let for_str = Statements::For.to_string();
    let if_str = Statements::If.to_string();
    let else_str = Statements::Else.to_string();
    let elseif = Statements::ElseIf.to_string();
    let switch = Statements::Switch.to_string();
    let case = Statements::Case.to_string();
    let default_str = Statements::Default.to_string();
    let function = Statements::Function.to_string();
    let close = Statements::Close.to_string();

    match data {
        while_str => {
            Ok(TokenType::Statements(Statements::While))
        },
        for_str => {
            Ok(TokenType::Statements(Statements::For))
        },
        if_str => {
            Ok(TokenType::Statements(Statements::If))
        }
        else_str => {
            Ok(TokenType::Statements(Statements::Else))
        }
        elseif => {
            Ok(TokenType::Statements(Statements::ElseIf))
        }
        switch => {
            Ok(TokenType::Statements(Statements::Switch))
        }
        case => {
            Ok(TokenType::Statements(Statements::Case))
        }
        default_str => {
            Ok(TokenType::Statements(Statements::Default))
        }
        function => {
            Ok(TokenType::Statements(Statements::Function))
        }
        close => {
            Ok(TokenType::Statements(Statements::Close))
        }
        _ => {
            Err(false)
        }
    }
}

fn arithmetic_token(data: &str) -> Result<TokenType, bool> {
    let plus = ArithmeticOperator::Plus.to_string();
    let minus = ArithmeticOperator::Minus.to_string();
    let times = ArithmeticOperator::Times.to_string();
    let power = ArithmeticOperator::Power.to_string();

    match data {
        plus => {
            Ok(TokenType::ArithmeticOperator(ArithmeticOperator::Plus))
        }
        minus => {
            Ok(TokenType::ArithmeticOperator(ArithmeticOperator::Minus))
        }
        times => {
            Ok(TokenType::ArithmeticOperator(ArithmeticOperator::Times))
        }
        power => {
            Ok(TokenType::ArithmeticOperator(ArithmeticOperator::Power))
        }
        _ => {
            Err(false)
        }
    }
}

fn conditional_token(data: &str) -> Result<TokenType, bool> {
    let and = Conditional::And.to_string();
    let not = Conditional::Not.to_string();
    let or = Conditional::Not.to_string();
    let xor = Conditional::Xor.to_string();
    let true_str = Conditional::True.to_string();
    let false_str = Conditional::False.to_string();
    let less = Conditional::Less.to_string();
    let greater = Conditional::Greater.to_string();
    let equals = Conditional::Equals.to_string();
    let less_eq = Conditional::LessEq.to_string();
    let great_eq = Conditional::GreaterEq.to_string();
    let not_eq = Conditional::NotEq.to_string();

    match data {
        and => {
            Ok(TokenType::Conditional(Conditional::And))
        },
        not => {
            Ok(TokenType::Conditional(Conditional::Not))
        },
        or => {
            Ok(TokenType::Conditional(Conditional::Or))
        },
        xor => {
            Ok(TokenType::Conditional(Conditional::Xor))
        },
        true_str => {
            Ok(TokenType::Conditional(Conditional::True))
        },
        false_str => {
            Ok(TokenType::Conditional(Conditional::False))
        },
        less => {
            Ok(TokenType::Conditional(Conditional::Less))
        },
        greater => {
            Ok(TokenType::Conditional(Conditional::Greater))
        },
        equals => {
            Ok(TokenType::Conditional(Conditional::Equals))
        },
        less_eq => {
            Ok(TokenType::Conditional(Conditional::LessEq))
        },
        great_eq => {
            Ok(TokenType::Conditional(Conditional::GreaterEq))
        },
        not_eq => {
            Ok(TokenType::Conditional(Conditional::NotEq))
        },
        _ => {
            Err(false)
        },
    }
}

fn tokenize_string(data: &str) -> TokenType {
    let statement = statement_token(data);

    if let Ok(token_statement) = statement {
        return token_statement
    }
    else {
        let arithmetic = arithmetic_token(data);

        if let Ok(token_arithmetic) = arithmetic {
            return token_arithmetic
        }
        else {
            let conditional = conditional_token(data);

            if let Ok(token_conditional) = conditional {
                return token_conditional
            }
            else {

                let primitive = tokenize_type(data);

                if let Ok(token_primive) = primitive {
                    return token_primive
                }
                else {
                    return TokenType::Identifier(data.to_string())
            }
        }
    }
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
        tokens.append(TokenType::NewLine);

    }

    return tokens
}
