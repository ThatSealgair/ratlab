/* Library file to set defaults for the language.
 */

//#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[allow(missing_docs)]
//#[serde(tag = "type")]
pub enum TokenType {
    // Logical or Boolean types
    Bool,
    Conditional(String),
    Arithmetic(String),
    // Primitive Types
    Char,
    Single,
    Double,
    Int8,
    Uint8,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
    // Syntax
    Tab,
    Space,
    SemiColon,
    Period,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    Equals,
    Identifier(String),
}

pub mod glitter {
    pub const INDENT: &str = "\t  ";
    pub const INDEX_OFFSET: i8 = 2;
    pub const COMMENT: &str = "%";
}

pub mod types {
    pub const CHAR: &str = "ch_ter";
    pub const BOOL: &str = "_log-boo_";
    pub const INT_8: &str = "8b-int";
    pub const UINT_8: &str = "unt_8b";
    pub const INT_16: &str = "2B-int";
    pub const UINT_16: &str = "unt_2B";
    pub const INT_32: &str = "8nybl-int";
    pub const UINT_32: &str = "unt_8nybl";
    pub const INT_64: &str = "8*8b-int";
    pub const UINT_64: &str = "unt_8*8b";
    pub const FLOAT: &str = "si_flo";
    pub const DOUBLE: &str = "do-flo";
    pub const STRING: &str = "&ret_hc";
}

pub mod conditional {
    pub const AND: &str = "&";
    pub const NOT: &str = "~";
    pub const OR: &str = "|";
    pub const XOR: &str = "xor";
    pub const TRUE: &str = "true";
    pub const FALSE: &str = "false";
    pub const LESS: &str = "<";
    pub const GREATER: &str = ">";
    pub const EQUALS: &str = "==";
    pub const LESS_EQ: &str = "<=";
    pub const GREATER_EQ: &str = ">=";
    pub const NOT_EQ: &str = "~=";
}

pub mod math_operator {
    pub const PLUS: &str = "+";
    pub const MINUS: &str = "-";
    pub const TIMES: &str = "*";
    pub const DIVIDE: &str = "/";
    pub const POWER: &str = "^";
    // To be constructed later from period
    // pub const ELEMENT_WISE: &str = ".";
}

pub mod statements {
    pub const WHILE: &str = "while";
    pub const FOR: &str = "for";
    pub const IF: &str = "if";
    pub const ELIF: &str = "elseif";
    pub const ELSE: &str = "else";
    pub const SWITCH: &str = "switch";
    pub const CASE: &str = "case";
    pub const DEFAULT: &str = "otherwise";
    pub const FN: &str = "function";
    pub const CLOSE: &str = "end";
}

pub mod syntax {
    pub const TAB: char = '\t';
    pub const SPACE: char = ' ';
    pub const SEMI_COLON: char = ';';
    pub const PERIOD: char = '.';
    pub const LEFT_BRACE: char = '(';
    pub const RIGHT_BRACE: char = ')';
    pub const LEFT_BRACKET: char = '{';
    pub const RIGHT_BRACKET: char = '}';
    pub const COLON: char = ':';
    pub const COMMA: char = ',';
    pub const EQUALS: char = '=';
}
