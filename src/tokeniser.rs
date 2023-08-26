/// tokeniser for ratlab

use crate::header::*;

fn tokenize_type(data: &str) -> Result<TokenType, bool> {
    use crate::header::primatives::*;

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
    use crate::header::statements::*;

    match data {
        WHILE => {
            Ok(TokenType::Statements(Statements::While))
        },
        FOR => {
            Ok(TokenType::Statements(Statements::For))
        },
        IF => {
            Ok(TokenType::Statements(Statements::If))
        }
        ELSE => {
            Ok(TokenType::Statements(Statements::Else))
        }
        ELIF => {
            Ok(TokenType::Statements(Statements::ElseIf))
        }
        SWITCH => {
            Ok(TokenType::Statements(Statements::Switch))
        }
        CASE => {
            Ok(TokenType::Statements(Statements::Case))
        }
        DEFAULT => {
            Ok(TokenType::Statements(Statements::Default))
        }
        FN => {
            Ok(TokenType::Statements(Statements::Function))
        }
        CLOSE => {
            Ok(TokenType::Statements(Statements::Close))
        }
        _ => {
            Err(false)
        }
    }
}

fn arithmetic_token(data: &str) -> Result<TokenType, bool> {
    use crate::header::arithmetic_operator::*;

    match data {
        PLUS => {
            Ok(TokenType::ArithmeticOperator(ArithmeticOperator::Plus))
        }
        MINUS => {
            Ok(TokenType::ArithmeticOperator(ArithmeticOperator::Minus))
        }
        TIMES => {
            Ok(TokenType::ArithmeticOperator(ArithmeticOperator::Times))
        }
        POWER => {
            Ok(TokenType::ArithmeticOperator(ArithmeticOperator::Power))
        }
        _ => {
            Err(false)
        }
    }
}

fn conditional_token(data: &str) -> Result<TokenType, bool> {
    use crate::header::conditional::*;

    match data {
        AND => {
            Ok(TokenType::Conditional(Conditional::And))
        },
        NOT => {
            Ok(TokenType::Conditional(Conditional::Not))
        },
        OR => {
            Ok(TokenType::Conditional(Conditional::Or))
        },
        XOR => {
            Ok(TokenType::Conditional(Conditional::Xor))
        },
        TRUE => {
            Ok(TokenType::Conditional(Conditional::True))
        },
        FALSE => {
            Ok(TokenType::Conditional(Conditional::False))
        },
        LESS => {
            Ok(TokenType::Conditional(Conditional::Less))
        },
        GREATER => {
            Ok(TokenType::Conditional(Conditional::Greater))
        },
        EQUALS => {
            Ok(TokenType::Conditional(Conditional::Equals))
        },
        LESS_EQ => {
            Ok(TokenType::Conditional(Conditional::LessEq))
        },
        GREATER_EQ => {
            Ok(TokenType::Conditional(Conditional::GreaterEq))
        },
        NOT_EQ => {
            Ok(TokenType::Conditional(Conditional::NotEq))
        },
        _ => {
            Err(false)
        },
    }
}

fn tokenize_string(input: String) -> TokenType {
    let data: &str = &input;
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
}


fn tokenize(data: Vec<String>) -> Vec<Vec<TokenType>> {
    use crate::header::syntax::*;

    let mut array: Vec<TokenType> = Vec::new();
    let mut tokens: Vec<Vec<TokenType>> = Vec::new();

    for line in data.iter() {
        let mut is_token = false;
        let mut token = Vec::new();

        for ch in line.chars() {
            match ch {
                TAB => {
                    if is_token {
                        array.push(
                            tokenize_string(token
                                .clone()
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    let symbol = Syntax::Tab;
                    array.push(TokenType::Syntax(symbol));
                },
                SPACE => {
                    if is_token {
                        array.push(
                            tokenize_string(token
                                .clone()
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    array.push(TokenType::Syntax(Syntax::Space));
                },
                SEMI_COLON => {
                    if is_token {
                        array.push(
                            tokenize_string(token
                                .clone()
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    array.push(TokenType::Syntax(Syntax::SemiColon));
                },
                COLON => {
                    if is_token {
                        array.push(
                            tokenize_string(token
                                .clone()
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    array.push(TokenType::Syntax(Syntax::Colon));
                },
                PEROID => {
                    if is_token {
                        array.push(
                            tokenize_string(token
                                .clone()
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    array.push(TokenType::Syntax(Syntax::Peroid));
                },
                COMMA => {
                    if is_token {
                        array.push(
                            tokenize_string(token
                                .clone()
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    array.push(TokenType::Syntax(Syntax::Comma));
                },
                EQUALS => {
                    if is_token {
                        array.push(
                            tokenize_string(token
                                .clone()
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    array.push(TokenType::Syntax(Syntax::Equals));
                },
                LEFT_BRACE => {
                    if is_token {
                        array.push(
                            tokenize_string(token
                                .clone()
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    array.push(TokenType::Syntax(Syntax::LeftBrace));
                },
                RIGHT_BRACE => {
                    if is_token {
                        array.push(
                            tokenize_string(token
                                .clone()
                                .into_iter()
                                .collect::<String>())
                        );
                        array.clear();
                        is_token = false;
                    }
                    array.push(TokenType::Syntax(Syntax::RightBrace));
                },
                LEFT_BRACKET => {
                    if is_token {
                        array.push(
                            tokenize_string(token
                                .clone()
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    array.push(TokenType::Syntax(Syntax::LeftBracket));
                },
                RIGHT_BRACKET => {
                    if is_token {
                        array.push(
                            tokenize_string(token
                                .clone()
                                .into_iter()
                                .collect::<String>())
                        );
                        token.clear();
                        is_token = false;
                    }
                    array.push(TokenType::Syntax(Syntax::RightBracket));
                },
                _ => {
                    is_token = true;
                    token.push(ch.clone());
                },
            }
        }
        tokens.push(array.clone());
        array.clear();
    }

    return tokens
}
