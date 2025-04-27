use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::sync::Mutex;

//TODO: i dont't think this may be necessary here
pub type Type = str;

//NOTE: I get why using Lazy but i do not understand it at a deep level read more about it
static COMMANDS: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub fn add_command(name: &str) {
    let mut kws = COMMANDS.lock().unwrap();
    kws.insert(name.to_string());
}

#[derive(Debug)]
pub struct Token<'a> {
    pub typ: &'a Type,
    //TODO: we ara going with string to avoid the double borrow error
    // look into taking a public trait instead it it proves to be the better case
    pub literal: String,
}

impl<'a> Token<'a> {
    /// Creates a new token
    pub fn new(tp: Tokens<'a>) -> Self {
        match tp {
            Tokens::Illegal(s) => {
                return Token {
                    typ: ILLEGAL,
                    literal: format!("{}", s),
                };
            }
            Tokens::Ident(s) => {
                return Token {
                    typ: IDENT,
                    literal: String::from(s),
                };
            }
            Tokens::DoubleDash => {
                return Token {
                    typ: DOUBLE_DASH,
                    literal: String::from(DOUBLE_DASH),
                };
            }
            Tokens::Dash => {
                return Token {
                    typ: DASH,
                    literal: String::from(DASH),
                };
            }
            Tokens::EOF => {
                return Token {
                    typ: EOF,
                    literal: String::from(EOF),
                };
            }
        }
    }

    pub fn new_ident(typ: &'a Type, literal: String) -> Self {
        return Token { typ, literal };
    }
}

pub enum Tokens<'a> {
    Illegal(char),
    Ident(&'a Type),
    DoubleDash,
    Dash,
    EOF,
}

pub const ILLEGAL: &Type = "ILLEGAL";
pub const IDENT: &Type = "IDENT";
pub const DOUBLE_DASH: &Type = "--";
pub const DASH: &Type = "-";
pub const EOF: &Type = "EOF";

//#[cfg(test)]
//mod tests {
//    use super::*;
//}
