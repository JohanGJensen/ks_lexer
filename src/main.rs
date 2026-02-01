use std::{fs, io};

mod tokens;
use tokens::{Token, TokenType};

struct Lexer {
    input: Vec<char>,
    tokens: Vec<Token>,
    index: usize,
}

impl Lexer {
    /**
     * Creates a new Lexer instance with the given input characters.
     */
    fn new(input: Vec<char>) -> Self {
        Self {
            input,
            tokens: Vec::new(),
            index: 0,
        }
    }

    /**
     * Returns the next character from the input, or None if the end is reached.
     */
    fn next_char(&mut self) -> Option<char> {
        if self.index >= self.input.len() {
            None
        } else {
            let char = self.input[self.index];
            self.index += 1;
            Some(char)
        }
    }

    fn preview_next_char(&self) -> Option<char> {
        if self.index >= self.input.len() {
            None
        } else {
            Some(self.input[self.index])
        }
    }

    /**
     * Returns the current character without advancing the index.
     */
    fn get_character(&self) -> Option<&char> {
        self.input.get(self.index)
    }

    /**
     * Keyword are kodesprog unique identifiers for functions, variables and similar
     */
    fn lex_keyword(&mut self, first_char: char) {
        let mut keyword = String::new();
        keyword.push(first_char);

        while let Some(&c) = self.get_character() {
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

            if keyword == "hvis" {
                self.tokens.push(Token { token: TokenType::If, value: keyword });
                break;
            }

            if keyword == "eller" {
                self.tokens.push(Token { token: TokenType::Else, value: keyword });
                break;
            }

            if keyword == "returner" {
                self.tokens.push(Token { token: TokenType::Return, value: keyword });
                break;
            }

            if c == '=' || c == ' ' || c == '(' || c == ';' {
                self.tokens.push(Token { token: TokenType::Identifier, value: keyword });
                break;
            }

            keyword.push(c);
            self.next_char();
        } 
    }

    /**
     * Lexes a string value, so ascii alpha-numeric characters.
     */
    fn lex_string(&mut self) {
        let mut ident = String::new();

        while let Some(&c) = self.get_character() {
            if c == '"' || c == '\'' {
                self.tokens.push(Token { token: TokenType::String, value: ident });

                self.next_char();
                break;
            }

            ident.push(c);
            self.next_char();
        }

        // skip closing quote
        if self.get_character() == Some(&'"') || self.get_character() == Some(&'\'') {
            self.next_char();
        }
    }

    /**
     * Lexes an integer literal.
     */
    fn lex_integer(&mut self, first_char: char) {
        let mut integer = String::new();
        integer.push(first_char);

        while let Some(&b) = self.get_character() {
            if !b.is_ascii_digit() {
                self.tokens.push(Token { token: TokenType::Number, value: integer });
                break;
            }

            integer.push(b as char);
            self.next_char();
        }
    }

    /**
     * Lexes an operator character and adds the corresponding token to the tokens list.
     */
    fn lex_operator(&mut self, char: char) {
        let chr = char.to_string();

        if char == '!' && self.preview_next_char() == Some('=') {
            let not_equal_char = format!("{}{}", chr, self.next_char().unwrap());
            self.tokens.push(Token { token: TokenType::NotEqual, value: not_equal_char });
            return;
        }

        if char == '=' {
            if self.preview_next_char() == Some('=') {
                let equal_char = format!("{}{}", chr, self.next_char().unwrap());

                self.tokens.push(Token { token: TokenType::Equal, value: equal_char });
                return;
            }

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

    /**
     * Tokenizes the input characters into tokens.
     */
    fn tokenize_character_input(&mut self) {
        while let Some(char) = self.next_char() {
            // log character being processed
            println!("Processing character: {}", char);
            match char {
                ' ' => continue,
                'a'..='z'|'A'..='Z' => self.lex_keyword(char),
                '"'|'\'' => self.lex_string(),
                '0'..='9'=> self.lex_integer(char),
                '!'|'='|';'|':'|'+'|'-'|'*'|'/' => self.lex_operator(char),
                '{' => self.tokens.push(Token { token: TokenType::OpenBracket, value: char.to_string() }),
                '}' => self.tokens.push(Token { token: TokenType::CloseBracket, value: char.to_string() }),
                '(' => self.tokens.push(Token { token: TokenType::OpenParen, value: char.to_string() }),
                ')' => self.tokens.push(Token { token: TokenType::CloseParen, value: char.to_string() }),
                _ => continue,
            }
        }

        self.tokens.push(Token { token: TokenType::EOF, value: String::new() });
    }

    /**
     * Prints the list of tokens for debugging purposes.
     */
    fn print_tokens(&self) {
        for t in &self.tokens {
            println!("token: {:?}, value: {:?}", t.token, t.value);
        }
    }

    /** 
     * Returns a reference to the list of tokens.
     */
    fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    fn output_tokens_file(&self, path: &str) -> io::Result<()> {
        let mut file = fs::File::create(path)?;

        for t in &self.tokens {
            use std::io::Write;
            writeln!(file, "token: {:?}, value: {:?}", t.token, t.value)?;
        }

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let main_file = fs::read_to_string("../app/index.ks").expect("could not find file");   
    let mut lexer = Lexer::new(main_file.chars().collect());

    lexer.tokenize_character_input();
    lexer.print_tokens();
    lexer.get_tokens();

    lexer.output_tokens_file("./tmp/tokens.txt")?;

    Ok(())
}
