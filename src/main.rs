use std::{fs, io};

#[derive(Debug)]
enum TokenType {
    // identifiers
    StringIdentifier,
    NumberIdentifier,
    // variables & keywords
    Function,
    FunctionScopeStart,
    FunctionScopeEnd,
    StringVar,
    IntVar,
    VarName,
    // operators
    ParentLeft,
    ParentRight,
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

struct Token {
    token: TokenType,
    value: String,
}

struct Lexer {
    input: Vec<char>,
    tokens: Vec<Token>,
    index: usize
}

impl Lexer {
    fn new(input: Vec<char>) -> Self {
        Self {
            input: input,
            tokens: Vec::new(),
            index: 0,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.index >= self.input.len() {
            None
        } else {
            let char = self.input[self.index];
            self.index += 1;
            Some(char)
        }
    }

    fn get_char(&self) -> Option<&char> {
        self.input.get(self.index)
    }

    /**
     * Keyword are kodesprog unique identifiers for functions, variables and similar
     */
    fn lex_keyword(&mut self, first_char: char) {
        let mut keyword = String::new();
        keyword.push(first_char);

        while let Some(&c) = self.get_char() {
            if keyword == "string" {
                self.tokens.push(Token { token: TokenType::StringVar, value: keyword });
                break;
            }
            
            if keyword == "int" {
                self.tokens.push(Token { token: TokenType::IntVar, value: keyword });
                break;
            }

            if keyword == "funktion" {
                self.tokens.push(Token { token: TokenType::Function, value: keyword });
                break;
            }

            if c == '=' || c == ' ' || c == '(' {
                self.tokens.push(Token { token: TokenType::VarName, value: keyword });
                break;
            }

            keyword.push(c);
            self.next_char();
        } 
    }

    /**
     * Lexes a string value, so ascii alpha-numeric characters.
     */
    fn lex_string(&mut self, first_char: char) {
        let mut ident = String::new();

        self.tokens.push(Token { token: TokenType::QuoteMark, value: first_char.to_string() });
        
        while let Some(&c) = self.get_char() {
            if c == '"' || c == '\'' {
                self.tokens.push(Token { token: TokenType::StringIdentifier, value: ident });
                self.tokens.push(Token { token: TokenType::QuoteMark, value: first_char.to_string() });

                self.next_char();
                break;
            }

            ident.push(c);
            self.next_char();
        }
    }

    /**
     * Lexes digits and numeric values
     */
    fn lex_integer(&mut self, first_char: char) {
        let mut integer = String::new();
        integer.push(first_char);

        while let Some(&b) = self.get_char() {
            if !b.is_ascii_digit() {
                self.tokens.push(Token { token: TokenType::NumberIdentifier, value: integer });
                break;
            }

            integer.push(b as char);
            self.next_char();
        }
    }

    /**
     * Lexes various operator characters. Mathematical and expressional.
     */
    fn lex_operator(&mut self, char: char) {
        let chr = char.to_string();

        if char == '=' {
            self.tokens.push(Token { token: TokenType::Assign, value: chr });
            return;
        }

        if char == ';' {
            self.tokens.push(Token { token: TokenType::SemiColon, value: chr });
            return;
        }

        if char == ':' {
            self.tokens.push(Token { token: TokenType::Colon, value: chr });
            return;
        }

        if char == '+' {
            self.tokens.push(Token { token: TokenType::Plus, value: chr });
            return;
        }

        if char == '-' {
            self.tokens.push(Token { token: TokenType::Minus, value: chr });
            return;
        }

        if char == '*' {
            self.tokens.push(Token { token: TokenType::Multiply, value: chr });
            return;
        }

        if char == '/' {
            self.tokens.push(Token { token: TokenType::Divide, value: chr });
            return;
        }  
    }

    fn tokenize_chars(&mut self) {
        while let Some(char) = self.next_char() {
            match char {
                ' ' => continue,
                'a'..='z'|'A'..='Z' => self.lex_keyword(char),
                '"'|'\'' => self.lex_string(char),
                '0'..='9'=> self.lex_integer(char),
                '='|';'|':'|'+'|'-'|'*'|'/' => self.lex_operator(char),
                '{' => self.tokens.push(Token { token: TokenType::FunctionScopeStart, value: char.to_string() }),
                '}' => self.tokens.push(Token { token: TokenType::FunctionScopeEnd, value: char.to_string() }),
                '(' => self.tokens.push(Token { token: TokenType::ParentLeft, value: char.to_string() }),
                ')' => self.tokens.push(Token { token: TokenType::ParentRight, value: char.to_string() }),
                _ => continue, 
            }
        }

        self.tokens.push(Token { token: TokenType::EOF, value: String::new() });
    }

    fn print_tokens(&self) {
        for t in &self.tokens {
            println!("token: {:?}", t.token);
            println!("value: {:?}", t.value);
        }
    }
}

fn main() -> io::Result<()> {
    let main_file = fs::read_to_string("../app/func.ks").expect("could not find file");   
    let mut lexer = Lexer::new(main_file.chars().collect());

    lexer.tokenize_chars();
    lexer.print_tokens();

    Ok(())
}
