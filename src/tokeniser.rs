/// tokeniser for ratlab

mod header;

fn tokenize_type(data: &str) -> TokenType {
  use types::*;

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
    CHAR | BOOL | INT_8 | UINT_8 | UINT_8 | INT_16 | | UINT_16 | INT_32 | UINT_32 | INT_64 | UINT_64 | FLOAT | DOUBLE | STRING {
        return tokenize_type(data);
    },
    AND | NOT | OR | XOR | TRUE | FALSE | LESS | GREATER | EQUALS | LESS_EQ | GREATER_EQ | NOT_EQ {
        return tokenize_conditionals(data);
    },
    PLUS, MINUS, TIMES, DIVIDE, POWER, ELEMENT_WISE {
      return tokenize_arithmetic(data)
    },
    WHILE, FOR, IF, ELIF, ELSE, SWITCH, CASE, DEFAULT, FN, CLOSE {
      return tokenize_statement(data)
    },
    _ => return TokenType::Identifier(data.to_string),
  } 
}


fn tokenize(data: &str) -> Result<(Vec<TokenType>) {
  let mut tokens = Vec::new();
  let mut line = Vec::new()

  for ch in data.chars() {
    let mut wasWhitespace = false;
    let mut whitespaceCount = 0;
    
    if (ch.is_whitespace()) {
      if (!wasWhitespace && !tokens.is_empty()) {
        tokens.push(&line.into_iter().collect()::<String>());
        line.clear();
      }

      whitespaceCount += 1;
      wasWhitespace = true;
    }
    else {
      if (wasWhitespace) {
        if (whitespaceCount == 6) {
          tokens.push(TokenType::Indentation(true));
        }
        else {
          tokens.push(TokenType::Indentation(false));
        }
        whitespaceCount = 0;
        wasWhitespace = false;
      }

      line.push(ch);
    }
  }
  
  return tokens;
}
