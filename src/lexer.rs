use crate::error::Diagnostic;
use crate::token::{Token, TokenKind};

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    start_line: usize,
    start_col: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            start_line: 1,
            start_col: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).copied()?;
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(c)
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn take_while(&mut self, out: &mut String, pred: impl Fn(char) -> bool) {
        while let Some(c) = self.peek() {
            if !pred(c) {
                break;
            }
            out.push(c);
            self.advance();
        }
    }

    fn either(&mut self, next: char, matched: TokenKind, otherwise: TokenKind) -> TokenKind {
        if self.match_char(next) {
            matched
        } else {
            otherwise
        }
    }

    fn doubled(&mut self, c: char, kind: TokenKind) -> Result<TokenKind, Diagnostic> {
        if self.match_char(c) {
            Ok(kind)
        } else {
            Err(self.error(format!("unexpected '{}', did you mean '{}{}'", c, c, c)))
        }
    }

    fn error(&self, message: impl Into<String>) -> Diagnostic {
        Diagnostic {
            line: self.start_line,
            col: self.start_col,
            message: message.into(),
        }
    }

    fn token(&self, kind: TokenKind) -> Token {
        Token {
            kind,
            line: self.start_line,
            col: self.start_col,
        }
    }

    fn skip_trivia(&mut self) {
        loop {
            match self.peek() {
                Some(' ') | Some('\r') | Some('\n') => {
                    self.advance();
                }
                Some('/') if self.peek_next() == Some('/') => {
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }

    fn number(&mut self, first: char) -> Result<TokenKind, Diagnostic> {
        let mut text = String::from(first);
        self.take_while(&mut text, |c| c.is_ascii_digit());

        if self.peek() == Some('.') {
            if !self.peek_next().is_some_and(|d| d.is_ascii_digit()) {
                return Err(
                    self.error("A number can't end with '.', put digits after it, or delete it")
                );
            }
            text.push('.');
            self.advance();
            self.take_while(&mut text, |c| c.is_ascii_digit());
            return Ok(TokenKind::Float(text));
        }

        Ok(TokenKind::Int(text))
    }

    fn identifier(&mut self, first: char) -> TokenKind {
        let mut text = String::from(first);
        self.take_while(&mut text, |c| c.is_alphanumeric() || c == '_');

        keyword(&text).unwrap_or(TokenKind::Identifier(text))
    }

    pub fn next_token(&mut self) -> Result<Token, Diagnostic> {
        self.skip_trivia();
        self.start_line = self.line;
        self.start_col = self.col;

        let c = match self.advance() {
            Some(c) => c,
            None => {
                return Ok(self.token(TokenKind::Eof));
            }
        };

        let kind = match c {
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,

            '+' => self.either('=', TokenKind::PlusEqual, TokenKind::Plus),
            '-' => self.either('=', TokenKind::MinusEqual, TokenKind::Minus),
            '/' => self.either('=', TokenKind::SlashEqual, TokenKind::Slash),
            '\\' => self.either('=', TokenKind::BackSlashEqual, TokenKind::BackSlash),
            '%' => self.either('=', TokenKind::PercentEqual, TokenKind::Percent),
            '=' => self.either('=', TokenKind::EqualEqual, TokenKind::Equal),
            '!' => self.either('=', TokenKind::BangEqual, TokenKind::Bang),
            '<' => self.either('=', TokenKind::LessEqual, TokenKind::Less),
            '>' => self.either('=', TokenKind::GreaterEqual, TokenKind::Greater),

            '*' => {
                if self.match_char('*') {
                    self.either('=', TokenKind::StarStarEqual, TokenKind::StarStar)
                } else {
                    self.either('=', TokenKind::StarEqual, TokenKind::Star)
                }
            }

            '?' => self.doubled('?', TokenKind::QuestionQuestion)?,
            '&' => self.doubled('&', TokenKind::And)?,
            '|' => self.doubled('|', TokenKind::Or)?,

            d if d.is_ascii_digit() => self.number(d)?,
            a if a.is_alphabetic() || a == '_' => self.identifier(a),

            other => return Err(self.error(format!("unexpected character '{}'", other))),
        };

        Ok(self.token(kind))
    }
}

fn keyword(text: &str) -> Option<TokenKind> {
    Some(match text {
        "fn" => TokenKind::Fn,
        "if" => TokenKind::If,
        "elif" => TokenKind::Elif,
        "else" => TokenKind::Else,
        "while" => TokenKind::While,
        "for" => TokenKind::For,
        "return" => TokenKind::Return,
        "priv" => TokenKind::Priv,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "and" => TokenKind::And,
        "or" => TokenKind::Or,
        "not" => TokenKind::Not,
        "import" => TokenKind::Import,
        "as" => TokenKind::As,
        "break" => TokenKind::Break,
        "continue" => TokenKind::Continue,
        "pass" => TokenKind::Pass,
        "in" => TokenKind::In,
        "try" => TokenKind::Try,
        "catch" => TokenKind::Catch,
        "throw" => TokenKind::Throw,
        _ => return None,
    })
}
