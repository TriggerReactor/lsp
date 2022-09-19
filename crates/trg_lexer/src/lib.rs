mod cursor;
pub mod token;

#[cfg(test)]
mod tests;

use cursor::Cursor;
use token::LiteralKind;
use token::LiteralKind::*;
use token::Token;
use token::TokenKind;
use token::TokenKind::*;

/// Parses the first token from the provided input string.
#[inline]
pub fn first_token(input: &str) -> Token {
    debug_assert!(!input.is_empty());
    Cursor::new(input).advance_token()
}

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        if cursor.is_eof() {
            None
        } else {
            cursor.reset_len_consumed();
            Some(cursor.advance_token())
        }
    })
}

pub fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
    | '\u{000A}' // \n
    | '\u{000B}' // vertical tab
    | '\u{000C}' // form feed
    | '\u{000D}' // \r
    | '\u{0020}' // space

    // NEXT LINE from latin1
    | '\u{0085}'

    // Bidi markers
    | '\u{200E}' // LEFT-TO-RIGHT MARK
    | '\u{200F}' // RIGHT-TO-LEFT MARK

    // Dedicated whitespace characters from Unicode
    | '\u{2028}' // LINE SEPARATOR
    | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

pub fn is_id_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_' || c == '#'
}

impl Cursor<'_> {
    /// Parses a token from the input string.
    fn advance_token(&mut self) -> Token {
        let first_char = self.bump().unwrap();
        let token_kind = match first_char {
            // Slash, line or block comment.
            '/' => match self.first() {
                '/' => self.line_comment(),
                '*' => self.block_comment(),
                _ => Slash,
            },

            // Whitespace sequence.
            c if is_whitespace(c) => self.whitespace(),

            // Numeric literal.
            _c @ '0'..='9' => {
                let kind = self.number();

                Literal { kind }
            }
            ';' => Semi,
            ',' => Comma,
            '.' => Dot,
            '(' => OpenParen,
            ')' => CloseParen,
            '{' => OpenBrace,
            '}' => CloseBrace,
            '[' => OpenBracket,
            ']' => CloseBracket,
            '@' => At,
            '#' => Pound,
            '~' => Tilde,
            '?' => Question,
            ':' => Colon,
            '$' => Dollar,
            '=' => Eq,
            '!' => Bang,
            '<' => Lt,
            '>' => Gt,
            '&' => And,
            '|' => Or,
            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            '^' => Caret,
            '%' => Percent,

            // String literal.
            '"' => {
                let terminated = self.eat_double_quoted_string();
                let kind = Str { terminated };
                Literal { kind }
            }
            _ => todo!(),
        };

        Token::new(token_kind, self.len_consumed())
    }

    fn line_comment(&mut self) -> TokenKind {
        debug_assert!(self.prev() == '/' && self.first() == '/');
        self.bump();

        self.eat_while(|c| c != '\n');
        LineComment
    }

    fn block_comment(&mut self) -> TokenKind {
        debug_assert!(self.prev() == '/' && self.first() == '*');
        self.bump();

        let mut depth = 1usize;
        while let Some(c) = self.bump() {
            match c {
                '/' if self.first() == '*' => {
                    self.bump();
                    depth += 1;
                }
                '*' if self.first() == '/' => {
                    self.bump();
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                _ => (),
            }
        }

        BlockComment {
            terminated: depth == 0,
        }
    }

    fn whitespace(&mut self) -> TokenKind {
        debug_assert!(is_whitespace(self.prev()));
        self.eat_while(is_whitespace);
        Whitespace
    }

    fn number(&mut self) -> LiteralKind {
        debug_assert!('0' <= self.prev() && self.prev() <= '9');

        self.eat_decimal_digits();

        match self.first() {
            '.' if self.second() != '.' && !is_id_start(self.second()) => {
                self.bump();

                let mut empty_exponent = true;
                if self.first().is_digit(10) {
                    empty_exponent = false;
                    self.eat_decimal_digits();
                }

                Decimal { empty_exponent }
            }
            _ => Int,
        }
    }

    /// Eats double-quoted string and returns true
    /// if string is terminated
    fn eat_double_quoted_string(&mut self) -> bool {
        debug_assert!(self.prev() == '"');
        while let Some(c) = self.bump() {
            match c {
                '"' => {
                    return true;
                }
                '\\' if self.first() == '\\' || self.first() == '"' => {
                    // Bump again to skip escaped character.
                    self.bump();
                }
                _ => (),
            }
        }

        // End of file is reached.
        false
    }

    fn eat_decimal_digits(&mut self) -> bool {
        let mut has_digit = false;
        loop {
            match self.first() {
                '_' => {
                    self.bump();
                }
                '0'..='9' => {
                    has_digit = true;
                    self.bump();
                }
                _ => break,
            }
        }
        has_digit
    }
}
