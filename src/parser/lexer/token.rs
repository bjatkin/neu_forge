#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    EOF,
    Unknown,
    OpenParen,
    CloseParen,
    Plus,
    Minus,
    Multiply,
    Divide,
    Let,
    Identifier,
    Int,
    Float,
    Bool,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub loc: usize,    // the location where the token was found
    pub value: String, // the value of the token
    pub t_type: Type,  // the type of the token
}

impl Token {
    pub fn from_bytes(source: &[u8], loc: usize, t_type: Type) -> Self {
        match String::from_utf8(source.to_vec()) {
            Ok(v) => {
                return Token {
                    loc: loc,
                    value: v,
                    t_type: t_type,
                };
            }
            Err(e) => panic!("invalid utf8 value {e}"),
        }
    }

    pub fn from_byte(byte: &u8, loc: usize, t_type: Type) -> Self {
        let char = *byte as char;
        return Token {
            loc: loc,
            value: String::from(char),
            t_type: t_type,
        };
    }
}
