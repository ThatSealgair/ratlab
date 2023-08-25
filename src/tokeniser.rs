/// tokeniser for ratlab
/// \

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
#[serde(tag = "type")]
pub enum TokenType {
  // Logical or Boolean types
  Bool(usize),
  Conditional(String),
  // Primitive Types
  Char(char),
  Single(f32),
  Double(f64),
  Iint8(usize),
  Uint8(usize),
  Iint16(usize),
  Uint16(usize),
  Iint32(usize),
  Uint32(usize),
  Iint64(usize),
  Uint64(usize),
  Identifier(String),
  Equals,
  SemiColon,
  Comma,
  Plus,
  Minus,
  End,
  Indentation(bool),
}

fn tokenize_string(data: &str) -> Result<(Vec<TokenType>, usize)> {
  let mut tokens = Vec::new();
  let mut line = Vec::new()
  let mut index = 0;

  for ch in data.chars() {
    let mut wasWhitespace = false;
    let mut whitespaceCount = 0;
    
    if (ch.is_whitespace()) {
      if (!wasWhitespace) {
        tokens.push(line);
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

  index += 1;
}
