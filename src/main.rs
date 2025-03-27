use core::str;
use std::{fs, io};

#[derive(Debug)]
#[derive(PartialEq)]
enum Token {
    StringIdentifier,
    NumberIdentifier,
    StringVar,
    IntVar,
    VarName,
    Assign,
    Plus,
    Minus,
    QuoteMark,
    Divide,
    Multiply,
    Colon,
    SemiColon,
    EOF,
}

#[derive(Debug)]
struct TokenizedType {
    token: Token,
    value: String,
}

struct Lexer {
    input: Vec<u8>,
    tokens: Vec<TokenizedType>,
    index: usize
}

impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            input: input.as_bytes().to_vec(),
            tokens: Vec::new(),
            index: 0,
        }
    }

    fn next_byte(&mut self) -> Option<u8> {
        if self.index >= self.input.len() {
            None
        } else {
            let byte = self.input[self.index];
            self.index += 1;
            Some(byte)
        }
    }

    fn lex_identifier(&mut self, first_byte: u8) {
        let mut ident = String::new();
        ident.push(first_byte as char);

        while let Some(&b) = self.input.get(self.index) {
            if (b == b'=' || b == b' ') && (ident != "string" && ident != "int") {
                self.tokens.push(TokenizedType { token: Token::VarName, value: ident });

                break;
            }

            if b.is_ascii_alphanumeric() {
                ident.push(b as char);
                self.index += 1;

                continue;
            }
            
            if ident == "string" {
                self.tokens.push(TokenizedType { token: Token::StringVar, value: ident });
                break;
            }
            
            if ident == "int" {
                self.tokens.push(TokenizedType { token: Token::IntVar, value: ident });
                break;
            }

            break;
        } 
    }

    fn lex_string(&mut self, first_byte: u8) {
        let mut ident = String::new();
        // ident.push(first_byte as char);

        self.tokens.push(TokenizedType { token: Token::QuoteMark, value: (first_byte as char).to_string() });
        
        while let Some(&b) = self.input.get(self.index) {
            if b == b'"' || b == b'\'' {
                self.tokens.push(TokenizedType { token: Token::StringIdentifier, value: ident });
                self.tokens.push(TokenizedType { token: Token::QuoteMark, value: (first_byte as char).to_string() });

                self.next_byte();
                break;
            }

            ident.push(b as char);
            self.index += 1;

            continue;
        }
    }

    fn lex_integer(&mut self, first_byte: u8) {
        let mut integer = String::new();
        integer.push(first_byte as char);

        while let Some(&b) = self.input.get(self.index) {
            if !b.is_ascii_digit() {
                self.tokens.push(TokenizedType { token: Token::NumberIdentifier, value: integer });
                break;
            }

            integer.push(b as char);
            self.index += 1;

            continue;
        }
    }

    fn tokenize_bytes(&mut self) {
        while let Some(byte) = self.next_byte() {
            match byte {
                b' ' => continue,
                b'a'..b'z'|b'A'..b'Z' => self.lex_identifier(byte),
                b'"'|b'\'' => self.lex_string(byte),
                48_u8..=57_u8=> self.lex_integer(byte),
                b'=' => self.tokens.push(TokenizedType { token: Token::Assign, value: (byte as char).to_string() }),
                b';' => self.tokens.push(TokenizedType { token: Token::SemiColon, value: (byte as char).to_string() }),
                b':' => self.tokens.push(TokenizedType { token: Token::Colon, value: (byte as char).to_string() }),
                b'+' => self.tokens.push(TokenizedType { token: Token::Plus, value: (byte as char).to_string() }),
                b'-' => self.tokens.push(TokenizedType { token: Token::Minus, value: (byte as char).to_string() }),
                b'*' => self.tokens.push(TokenizedType { token: Token::Multiply, value: (byte as char).to_string() }),
                b'/' => self.tokens.push(TokenizedType { token: Token::Divide, value: (byte as char).to_string() }),
                _ => continue, 
            }
        }

        self.tokens.push(TokenizedType { token: Token::EOF, value: String::new() });
    }

    fn print_tokens(&mut self) {
        for t in &self.tokens {
            println!("token: {:?}", t.token);
            println!("value: {:?}", t.value);
        }
    }
}

fn main() -> io::Result<()> {
    let main_file = fs::read_to_string("../app/variable.ks").expect("could not find file");
    
    for line in main_file.lines() {
        let mut lexer = Lexer::new(line);
        
        lexer.tokenize_bytes();
        lexer.print_tokens();
    }

    Ok(())
}
