/// tokeniser for ratlab

use crate::header;

macro_rules! assign_token {
    ($is_token:expr, $tokens:expr, $token:expr, $token_type:path) => {
        if $is_token {
            $tokens.push(&$token.into_iter().collect::<String>());
            $token.clear();
            $is_token = false;
        }
        $tokens.push($token_type);
    };
}

macro_rules! assign_token_two {
    ($is_token:expr, $tokens:expr, $token:expr, $($token_type:tt)*) => {
        if $is_token {
            $tokens.push(&$token.into_iter().collect::<String>());
            $token.clear();
            $is_token = false;
        }
        $tokens.push($token_type);
    };
}

fn tokenize_type(data: &str) -> TokenType {
    use header::types::*;

    match data {
        CHAR => return TokenType::Char,
        BOOL => return TokenType::Bool,
        INT_8 => return TokenType::Int8,
        UINT_8 => return TokenTyp::Uint8,
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

fn tokenize_conditionals(data: &str) -> TokenType {
  use conditional::*;

  return TokenType::Conditional(data.to_string())
}

fn tokenize_arithmetic(data: &str) -> TokenType {
  use math_operator::*;

  return TokenType::Arithmetic(data.to_string())
}


fn tokenize_string(data: &str) -> TokenType {
    use types::*;
    use conditional::*;
    use math_operator::*;
    use statements::*;

    match data {
        CHAR | BOOL | INT_8 | UINT_8 | UINT_8 | INT_16 | | UINT_16 | INT_32 | UINT_32 | INT_64 | UINT_64 | FLOAT | 
            DOUBLE | STRING => {
            return tokenize_type(data)
        },
        AND | NOT | OR | XOR | TRUE | FALSE | LESS | GREATER | EQUALS | LESS_EQ | GREATER_EQ | NOT_EQ => {
            return tokenize_conditionals(data)
        },
        PLUS | MINUS | TIMES | DIVIDE | POWER | ELEMENT_WISE => {
        return tokenize_arithmetic(data)
        },
        WHILE | FOR | IF | ELIF | ELSE | SWITCH | CASE | DEFAULT | FN | CLOSE => {
        return tokenize_statement(data)
        },
        _ => return TokenType::Identifier(data.to_string()),
    } 
}


fn tokenize(data: Vec<String>) -> Vec<TokenType> {
    let mut tokens = Vec::new();

    for line in data.iter() {
        let mut isToken = false;
        let mut hasTab = false;
        let mut spaceCount = 0;
        let mut token = Vec::new();

        for ch in line.chars() {
            match ch {
                TAB => {
                    assign_token(isToken, tokens, token, TokenType::Tab);
                },
                SPACE => {
                    assign_token_two(isToken, tokens, token, TokenType::Space);
                },
                _ => {
                    token.push(ch);
                },
            }
        }
    }

    return tokens
}
