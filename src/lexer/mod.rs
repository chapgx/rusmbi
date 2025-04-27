use crate::tokens::*;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Self {
            input,
            position: 0,
            read_position: 0,
            ch: '0',
        };
        l.read_char();
        return l;
    }

    /// Reads next character in the input
    fn read_char(&mut self) {
        let mut chars = self.input.chars();

        if let Some(c) = chars.nth(self.read_position) {
            self.ch = c;
        } else {
            self.ch = '0'
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    /// Returns next token in the stream
    pub fn next(&mut self) -> Token<'_> {
        println!("made it here");
        self.eat_white_space();

        let t: Token;
        match self.ch {
            '0' => {
                t = Token::new(Tokens::EOF);
            }
            '-' => {
                if self.peek_next() == '-' {
                    t = Token::new(Tokens::DoubleDash);
                    self.read_char();
                } else {
                    t = Token::new(Tokens::Dash);
                    //TODO: if single dash all the following characters are tokens
                }
            }
            _ => {
                if self.is_letter() {
                    let s = self.read_ident();
                    t = Token::new_ident(IDENT, String::from(s));
                } else {
                    t = Token::new(Tokens::Illegal(self.ch));
                }
            }
        }

        self.read_char();
        return t;
    }

    /// Looks up the next token in the stream if none is found returns EOF
    fn peek_next(&mut self) -> char {
        //NOTE: should i check the length to see if this maybe an out of
        //bounds exceptions if such thing exists in rust
        let mut chars = self.input.chars();
        if let Some(c) = chars.nth(self.read_position) {
            return c;
        } else {
            return '0';
        }
    }

    fn eat_white_space(&mut self) {
        while self.ch == ' ' {
            self.read_char();
        }
    }

    fn is_letter(&mut self) -> bool {
        match self.ch {
            'a'..'z' => return true,
            'A'..'Z' => return true,
            '_' => return true,
            _ => return false,
        }
    }

    fn read_ident(&mut self) -> &mut str {
        let pos = self.position;

        while self.is_letter() {
            self.read_char();
        }

        return &mut self.input[pos..self.position];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens;
    use crate::tokens::Token;

    #[test]
    fn test_lexer() {
        let input = String::from("rhombi --person val");
        let mut l = Lexer::new(input);

        let expected_tokens: Vec<Token> = vec![
            Token {
                typ: tokens::IDENT,
                literal: String::from("rhombi"),
            },
            Token {
                typ: tokens::DOUBLE_DASH,
                literal: String::from("--"),
            },
            Token {
                typ: tokens::IDENT,
                literal: String::from("person"),
            },
            Token {
                typ: tokens::IDENT,
                literal: String::from("val"),
            },
        ];

        for t in expected_tokens {
            let tok = l.next();
            println!("got = {:?}, expected = {:?}", tok, t);
            assert!(tok.typ == t.typ, "expected {}, got {}", t.typ, tok.typ);
            assert!(
                tok.literal == t.literal,
                "expected \"{}\", got \"{}\"",
                t.literal,
                tok.literal
            );
        }
    }
}
