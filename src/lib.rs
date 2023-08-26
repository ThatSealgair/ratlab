/* Standard library file for the ratlab compiler.
 */

// External module files to use
pub mod input;
pub mod tokeniser;
pub mod validation;
pub mod stdlib;

// Standard header contents to set defaults for the language.
pub mod header {
    #[derive(Clone)]
    pub enum TokenType {
        PrimitiveType(PrimitiveType),
        Syntax(Syntax),
        Statements(Statements),
        ArithmeticOperator(ArithmeticOperator),
        Conditional(Conditional),
        Identifier(String),
        NewLine,
    }


    impl TokenType {
        pub fn to_string(&self) -> &str {
            return "he he poo poo"
        }
    }

    #[derive(Copy, Clone)]
    pub enum PrimitiveType {
        BOOL,
        CHAR,
        SINGLE,
        DOUBLE,
        INT8,
        UINT8,
        INT16,
        UINT16,
        INT32,
        UINT32,
        INT64,
        UINT64,
    }

    #[derive(Copy, Clone)]
    pub enum Conditional {
        And,
        Not,
        Or,
        Xor,
        True,
        False,
        Less,
        Greater,
        Equals,
        LessEq,
        GreaterEq,
        NotEq,
    }

    impl Conditional {
        pub fn to_string(&self) -> &str {
            match self {
                Conditional::And => conditional::AND,
                Conditional::Not => conditional::NOT,
                Conditional::Or => conditional::OR,
                Conditional::Xor => conditional::XOR,
                Conditional::True => conditional::TRUE,
                Conditional::False => conditional::FALSE,
                Conditional::Less => conditional::LESS,
                Conditional::Greater => conditional::GREATER,
                Conditional::Equals => conditional::EQUALS,
                Conditional::LessEq => conditional::LESS_EQ,
                Conditional::GreaterEq => conditional::GREATER_EQ,
                Conditional::NotEq => conditional::NOT_EQ,
            }
        }
    }

    #[derive(Copy, Clone)]
    pub enum ArithmeticOperator {
        Plus,
        Minus,
        Times,
        Power,
    }

    impl ArithmeticOperator {
        pub fn to_string(&self) -> &str {
            match self {
                ArithmeticOperator::Plus => arithmetic_operator::PLUS,
                ArithmeticOperator::Minus => arithmetic_operator::MINUS,
                ArithmeticOperator::Times => arithmetic_operator::TIMES,
                ArithmeticOperator::Power => arithmetic_operator::POWER,
            }
        }
    }

    #[derive(Copy, Clone)]
    pub enum Statements {
        While,
        For,
        If,
        Else,
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
                Statements::While => statements::WHILE,
                Statements::For => statements::FOR,
                Statements::If => statements::IF,
                Statements::Else => statements::ELSE,
                Statements::ElseIf => statements::ELIF,
                Statements::Switch => statements::SWITCH,
                Statements::Case => statements::CASE,
                Statements::Default => statements::DEFAULT,
                Statements::Function => statements::FN,
                Statements::Close => statements::CLOSE,
            }
        }
    }

    #[derive(Copy, Clone)]
    pub enum Syntax {
        Tab,
        Space,
        SemiColon,
        Colon,
        Period,
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
                Syntax::Tab => syntax::TAB,
                Syntax::Space => syntax::SPACE,
                Syntax::SemiColon => syntax::SEMI_COLON,
                Syntax::Colon => syntax::COLON,
                Syntax::Period => syntax::PERIOD,
                Syntax::Comma => syntax::COMMA,
                Syntax::Equals => syntax::EQUALS,
                Syntax::LeftBrace => syntax::LEFT_BRACE,
                Syntax::RightBrace => syntax::RIGHT_BRACE,
                Syntax::LeftBracket => syntax::LEFT_BRACKET,
                Syntax::RightBracket => syntax::RIGHT_BRACKET,
            }
        }
    }

    pub mod syntax {
        pub const TAB: char = '\t';
        pub const SPACE: char = ' ';
        pub const SEMI_COLON: char = ';';
        pub const COLON: char = ':';
        pub const PERIOD: char = '.';
        pub const COMMA: char = ',';
        pub const EQUALS: char = '=';
        pub const LEFT_BRACE: char = '{';
        pub const RIGHT_BRACE: char = '}';
        pub const LEFT_BRACKET: char = '(';
        pub const RIGHT_BRACKET: char = ')';
    }

    pub mod glitter {
        pub const INDENT: &str = "\t  ";
        pub const INDEX_OFFSET: i8 = 2;
        pub const COMMENT: &str = "%";
        pub const START_COMMENT: &str = "%{";
        pub const END_COMMENT: &str = "%}";
    }

    pub mod primatives {
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

    pub mod arithmetic_operator {
        pub const PLUS: &str = "+";
        pub const MINUS: &str = "-";
        pub const TIMES: &str = "*";
        pub const DIVIDE: &str = "/";
        pub const POWER: &str = "^";
        pub const ELEMENT_WISE: &str = ".";
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
}
