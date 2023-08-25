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

impl From<String> for TokenType {
  fn from(other: String) -> TokenType {
    TokenType::Identifier(other)
  }
}

impl<'a> From<&'a str> for TokenType {
  fn from(other: &'a str) -> TokenType {
    TokenType::Identifier(other.to_string())
  }
}


