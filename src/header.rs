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
    Syntax(Syntax),

    Identifier(String),
}


impl TokenType {
    pub fn to_char(&self) -> &str {
        match self {
            Syntax::Tab => '\t',
            Syntax::Space => ' ',
            Syntax::SemiColon => ';',
            Syntax::Colon => ':',
            Syntax::Peroid => '.',
            Syntax::Comma => ',',
            Syntax::Equals => '=',
            Syntax::LeftBrace => '(',
            Syntax::RightBrace => ')',
            Syntax::LeftBracket => '{',
            Syntax::RightBracket => '}',
        }
    }
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

pub enum Conditional {
    And = "&",
    Not = "~",
    Or = "|",
    Xor = "xor",
    True = "true",
    False = "false",
    Less = "<",
    Greater = ">",
    Equals = Syntax.Equals + Syntax.Equals,
    LessEq = Less + Syntax.Equals,
    GreaterEq = Greater + Syntax.Equals,
    NotEq = Not + Syntax.Equals,
}

impl Syntax {
    pub fn to_char(&self) -> char {
        match self {
            Syntax::Tab => '\t',
            Syntax::Space => ' ',
            Syntax::SemiColon => ';',
            Syntax::Colon => ':',
            Syntax::Peroid => '.',
            Syntax::Comma => ',',
            Syntax::Equals => '=',
            Syntax::LeftBrace => '(',
            Syntax::RightBrace => ')',
            Syntax::LeftBracket => '{',
            Syntax::RightBracket => '}',
        }
    }
}

impl Syntax {
    pub fn to_char(&self) -> char {
        match self {
            Syntax::Tab => '\t',
            Syntax::Space => ' ',
            Syntax::SemiColon => ';',
            Syntax::Colon => ':',
            Syntax::Peroid => '.',
            Syntax::Comma => ',',
            Syntax::Equals => '=',
            Syntax::LeftBrace => '(',
            Syntax::RightBrace => ')',
            Syntax::LeftBracket => '{',
            Syntax::RightBracket => '}',
        }
    }
}

pub enum ArithmeticOperator {
    Plus = "+",
    Minus = "-",
    Times = "/",
    Power = "^",
}

impl ArithmeticOperator {
    pub fn to_char(&self) -> char {
        match self {
            ArithmeticOperator::Plus => '+',
            ArithmeticOperator::Minus => '-',
            ArithmeticOperator::Times => '/',
            ArithmeticOperator::Power => '^',
        }
    }
}


pub enum Statements {
    While,
    For,
    If,
    ElseIf,
    Switch,
    Case,
    Default,
    Function,
    Close,
}

impl Statements {
    pub fn to_string(&self) -> &str {
        match self {
            Statements::While => "while",
            Statements::For => "for",
            Statements::If => "if",
            Statements::ElseIf => "elseif",
            Statements::Switch => "switch",
            Statements::Case => "case",
            Statements::Default => "otherwise",
            Statements::Function => "function",
            Statements::Close => "end",
        }
    }
}

pub enum Syntax {
    Tab,
    Space,
    SemiColon,
    Colon,
    Peroid,
    Comma,
    Equals,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
}

impl Syntax {
    pub fn to_char(&self) -> char {
        match self {
            Syntax::Tab => '\t',
            Syntax::Space => ' ',
            Syntax::SemiColon => ';',
            Syntax::Colon => ':',
            Syntax::Peroid => '.',
            Syntax::Comma => ',',
            Syntax::Equals => '=',
            Syntax::LeftBrace => '(',
            Syntax::RightBrace => ')',
            Syntax::LeftBracket => '{',
            Syntax::RightBracket => '}',
        }
    }
}
