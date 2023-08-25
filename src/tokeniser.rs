/// tokeniser for ratlab

mod header;

fn tokenize_string(data: &str) -> TokenType {
  match data {
    types::CHAR => return TokenType::Char,
    types::BOOL => return TokenType::Bool,
    types::INT_8 => return TokenType::Int8,
    types::UINT_8 => return TokenType::Uint8,
    types::INT_16 => return TokenType::Int16,
    types::UINT_16 => return TokenType::Uint16,
    types::INT_32 => return TokenType::Int32,
    types::UINT_32 => return TokenType::Uint32,
    types::INT_64 => return TokenType::Int64,
    types::UINT_64 => return TokenType::Uint64,
    types::FLOAT => return TokenType::Single,
    types::DOUBLE => return TokenType::Double,
    _ => return TokenType::Identifier(data),
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
