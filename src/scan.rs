use std::str::Chars;
use std::iter::Peekable;
use crate::token::{TokenType, TokenValue, Token};

#[derive(Debug)]
pub struct Scanner<'a> {
    reader: Peekable<Chars<'a>>,
    current_char: char,
    line: u32,
    tokens: Vec<Token>,
}

impl<'a> Scanner<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            reader: content.chars().peekable(),
            current_char: '\u{0}',
            line: 0,
            tokens: Vec::new()
        }
    }
    
    pub fn scan(&mut self) -> Vec<Token> {
        while let Some(current_char) = self.reader.next() {
            self.current_char = current_char;
            match current_char {
                '{' => self.tokens.push(Token::new(TokenType::LBRACE, None)),
                '}' => self.tokens.push(Token::new(TokenType::RBRACE, None)),
                '[' => self.tokens.push(Token::new(TokenType::LBRACKET, None)),
                ']' => self.tokens.push(Token::new(TokenType::RBRACKET, None)),
                ',' => self.tokens.push(Token::new(TokenType::COMMA, None)),
                ':' => self.tokens.push(Token::new(TokenType::COLON, None)),
                ' ' => self.tokens.push(Token::new(TokenType::WHITESPACE, None)),
                't' | 'f' => {
                    let token = self.get_bool();
                    self.tokens.push(token);
                },
                'n' => {
                    let token = self.get_null();
                    self.tokens.push(token);
                },
                '"' => {
                    let token = self.get_string();
                    self.tokens.push(token);
                },
                '-' | '0'..='9' => {
                    let token = self.get_digit();
                    self.tokens.push(token);
                },
                '\t' | '\r' => continue,
                '\n' => self.increment_line(),
                _ => ()
            }
        }

        self.tokens.clone()
    }

    fn increment_line(&mut self) {
        self.line += 1;
    }

    fn get_digit(&mut self) -> Token {
        let mut digit = String::from(self.current_char);

        loop {
            let next = self.reader.peek().expect("expected right brace, got EOF");

            if next.is_digit(10) || *next == '.' {
                digit.push(*next);
                self.reader.next().unwrap();
            } else {
                break;
            }
        }

        let number_val: f64 = digit.parse().unwrap();
        Token::new(TokenType::NUMBER, Some(TokenValue::NUMBER(number_val)))
    }

    fn get_string(&mut self) -> Token {
        let mut string = String::new();

        loop {
            let next = self.reader.peek().expect("expected right brace, got EOF");

            if *next == '"' {
                self.reader.next().unwrap();
                break;
            } else if next.is_ascii() {
                string.push(*next);
                self.reader.next().unwrap();
            } else {
                break;
            }
        }

        Token::new(TokenType::STRING, Some(TokenValue::STRING(string)))
    }

    fn get_null(&mut self) -> Token {
       let remaining_null = ['u', 'l', 'l'];

       for ch in remaining_null {
           let next = self.reader.peek().expect("expected null value, got EOF");

           if *next == ch {
               self.reader.next().unwrap();
           } else {
               panic!("expected a null value, got {}", next)
           }
       }

       Token::new(TokenType::NULL, None)
    }

    fn get_bool(&mut self) -> Token {
       let remaining_true = ['r', 'u', 'e'];
       let remaining_false = ['a', 'l', 's', 'e'];

       let mut selected_bool = false;
       let selected: Vec<char> = if self.current_char == 't' {
           selected_bool = true;
           remaining_true.into()
       } else {
           remaining_false.into()
       };

       for ch in selected {
           let next = self.reader.peek().expect("expected a boolean value, got EOF");

           if *next == ch {
               self.reader.next().unwrap();
           } else {
               panic!("expected a boolean value, got {}", next)
           }
       }

       Token::new(TokenType::BOOL, Some(TokenValue::BOOL(selected_bool)))
    }
}
