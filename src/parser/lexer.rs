pub mod token;

use token::{Token, Type};

#[derive(Clone)]
pub struct Lexer<'a> {
    source: &'a [u8],
    idx: usize,
    next_token: Token,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let source = source.as_bytes();
        let (next_token, idx) = match_token(0, source);
        Lexer {
            source: source,
            idx: idx,
            next_token: next_token,
        }
    }

    pub fn peek(&self) -> Token {
        self.next_token.clone()
    }

    pub fn take(&mut self) -> Token {
        let tok = self.next_token.clone();
        if tok.t_type == Type::EOF {
            return tok;
        }

        let (next_token, skip) = match_token(self.idx, &self.source[self.idx..]);
        self.next_token = next_token;
        self.idx += skip;

        return tok;
    }
}

fn match_token(loc: usize, source: &[u8]) -> (Token, usize) {
    let mut start = 0;

    // skip over any whitespace characters to start
    for i in 0..source.len() {
        if is_white_space(&source[i]) {
            start += 1;
            continue;
        }
        break;
    }

    let source = &source[start..];
    if source.len() == 0 {
        let tok = Token {
            loc: start,
            value: String::from("EOF"),
            t_type: Type::EOF,
        };
        return (tok, start);
    }

    // look for single byte tokens
    match source[0] {
        b'(' => {
            let tok = Token::from_byte(&source[0], start + loc, Type::OpenParen);
            return (tok, start + 1);
        }
        b')' => {
            let tok = Token::from_byte(&source[0], start + loc, Type::CloseParen);
            return (tok, start + 1);
        }
        b'+' => {
            let tok = Token::from_byte(&source[0], start + loc, Type::Plus);
            return (tok, start + 1);
        }
        b'-' => {
            let tok = Token::from_byte(&source[0], start + loc, Type::Minus);
            return (tok, start + 1);
        }
        b'*' => {
            let tok = Token::from_byte(&source[0], start + loc, Type::Multiply);
            return (tok, start + 1);
        }
        b'/' => {
            let tok = Token::from_byte(&source[0], start + loc, Type::Divide);
            return (tok, start + 1);
        }
        _ => { /* continue on */ }
    };

    // look for multi-byte tokens
    match int_token(start + loc, source) {
        Some(t) => {
            let l = t.value.len();
            return (t, start + l);
        }
        None => { /* continue on */ }
    }

    match float_token(start + loc, source) {
        Some(t) => {
            let l = t.value.len();
            return (t, start + l);
        }
        None => { /* continue on */ }
    }

    match bool_token(start + loc, source) {
        Some(t) => {
            let l = t.value.len();
            return (t, start + l);
        }
        None => { /* continue on */ }
    }

    match keyword_or_identifier_token(loc, source) {
        Some(t) => {
            let l = t.value.len();
            return (t, start + l);
        }
        None => { /* continue on */ }
    }

    let tok = unknown_token(start + loc, source);
    let l = tok.value.len();
    return (tok, start + l);
}

const VALID_NUMBERS: &[u8] = "_0123456789".as_bytes();

fn int_token(loc: usize, source: &[u8]) -> Option<Token> {
    if !VALID_NUMBERS.contains(&source[0]) {
        return None;
    }

    for i in 1..source.len() {
        if !VALID_NUMBERS.contains(&source[i]) {
            let tok = Token::from_bytes(&source[0..i], loc, Type::Int);
            return Some(tok);
        }
    }

    let tok = Token::from_bytes(&source, loc, Type::Int);
    return Some(tok);
}

fn float_token(loc: usize, source: &[u8]) -> Option<Token> {
    // first byte must be a valid number
    if !VALID_NUMBERS.contains(&source[0]) {
        return None;
    }

    let mut leading_digits = 0;
    let mut trailing_digits = 0;
    let mut found_point = false;
    for i in 1..source.len() {
        if source[i] == b'.' && found_point {
            // if we found more than a single point, this is not a float
            return None;
        }
        if source[i] == b'.' && !found_point {
            found_point = true;
            continue;
        }
        if VALID_NUMBERS.contains(&source[i]) && !found_point {
            leading_digits += 1;
            continue;
        }
        if VALID_NUMBERS.contains(&source[i]) && found_point {
            trailing_digits += 1;
            continue;
        }

        // we found a non-decimal character byte
        if leading_digits > 0 && found_point && trailing_digits > 0 {
            let tok = Token::from_bytes(&source[0..i], loc, Type::Float);
            return Some(tok);
        }

        // we didnt have leading digits, a single decimal point, and trailing digits so this is
        // not a valid floating point literal
        return None;
    }

    if leading_digits > 0 && found_point && trailing_digits > 0 {
        let tok = Token::from_bytes(&source, loc, Type::Float);
        return Some(tok);
    }

    return None;
}

fn bool_token(loc: usize, source: &[u8]) -> Option<Token> {
    let true_bytes = "true".as_bytes();
    if source.starts_with(true_bytes) && source.len() == true_bytes.len() {
        let tok = Token::from_bytes(true_bytes, loc, Type::Bool);
        return Some(tok);
    }

    if source.starts_with(true_bytes) && is_white_space(&source[true_bytes.len()]) {
        let tok = Token::from_bytes(true_bytes, loc, Type::Bool);
        return Some(tok);
    }

    let false_bytes = "false".as_bytes();
    if source.starts_with(false_bytes) && source.len() == false_bytes.len() {
        let tok = Token::from_bytes(false_bytes, loc, Type::Bool);
        return Some(tok);
    }

    if source.starts_with(false_bytes) && is_white_space(&source[false_bytes.len()]) {
        let tok = Token::from_bytes(false_bytes, loc, Type::Bool);
        return Some(tok);
    }

    return None;
}

fn keyword_or_identifier_token(loc: usize, source: &[u8]) -> Option<Token> {
    let keyword = keyword_token(loc, source);
    let ident = ident_token(loc, source);
    match &keyword {
        Some(k) => match &ident {
            Some(i) => {
                if k.value.len() >= i.value.len() {
                    return keyword;
                } else {
                    return ident;
                }
            }
            None => return keyword,
        },
        None => match ident {
            Some(_) => return ident,
            None => return None,
        },
    };
}

fn keyword_token(loc: usize, source: &[u8]) -> Option<Token> {
    if source.starts_with("let".as_bytes()) {
        return Some(Token {
            loc: loc,
            value: String::from("let"),
            t_type: Type::Let,
        });
    };

    return None;
}

const VALID_IDENT_PREFIX: &[u8] = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
const VALID_IDENT_SUFFIX: &[u8] = "_0123456789".as_bytes();

fn ident_token(loc: usize, source: &[u8]) -> Option<Token> {
    if !VALID_IDENT_PREFIX.contains(&source[0]) {
        return None;
    }

    for i in 1..source.len() {
        if !VALID_IDENT_PREFIX.contains(&source[i]) && !VALID_IDENT_SUFFIX.contains(&source[i]) {
            let tok = Token::from_bytes(&source[0..i], loc, Type::Identifier);
            return Some(tok);
        }
    }

    let tok = Token::from_bytes(&source, loc, Type::Identifier);
    return Some(tok);
}

fn unknown_token(loc: usize, source: &[u8]) -> Token {
    for i in 0..source.len() {
        if is_white_space(&source[i]) {
            let tok = Token::from_bytes(&source[0..i], loc, Type::Unknown);
            return tok;
        }
    }

    let tok = Token::from_bytes(&source, loc, Type::Unknown);
    return tok;
}

fn is_white_space(byte: &u8) -> bool {
    *byte == b'\n' || *byte == b' ' || *byte == b'\t'
}
