///
/// Lexical analysis (tokenization)
/// 
/// The lexer takes a string of characters and converts it into a list of tokens.
/// 

#[derive(Debug, PartialEq)]
pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            position: 0,
        }
    }

    fn position(&self) -> usize {
        self.position
    }

    fn next(&mut self) -> Option<char> {
        let chr = self.peek();
        
        self.position += 1;

        chr
    }

    fn peek(&mut self) -> Option<char> {
        if self.position >= self.input.len() {
            return Some('\0');
        }

        if let Some(chr_as_str) = self.input.get(self.position..self.position+1) {
            return chr_as_str.chars().next();
        }

        return Some('\0');
    }

    pub fn scan(&mut self) -> Result<Vec::<Token>, String> {
        let mut symbol;
        let mut symbol_position;
        let mut token_list = Vec::<Token>::new();

        loop {
            symbol = self.peek().unwrap();
            symbol_position = self.position;

            match symbol
            {
                // Whitespace
                ' ' | '\t' | '\n' => {
                    self.next();
                },
                // Identifier
                'a'..='z' | 'A'..='Z' => {
                    let mut identifier = String::new();

                    while let Some(chr) = self.peek() {
                        if chr.is_alphanumeric() || chr == '_' {
                            identifier.push(chr);
                            self.next();
                        } else {
                            break;
                        }
                    }

                    token_list.push(Token {
                        token_type: lexeme_from_string(identifier),
                        line_number: 0,
                        char_start: symbol_position,
                        char_end: self.position(),
                    });
                },
                // Integer
                '0'..='9' => {
                    let mut integer = String::new();

                    while let Some(chr) = self.peek() {
                        if chr.is_numeric() {
                            integer.push(chr);
                            self.next();
                        } else {
                            break;
                        }
                    }

                    token_list.push(Token {
                        token_type: Lexeme::Integer(integer.parse::<i64>().unwrap()),
                        line_number: 0,
                        char_start: symbol_position,
                        char_end: self.position(),
                    });
                },
                // Binary Operator
                '+' | '-' | '*' | '/' | '%' => {                    
                    self.next();

                    token_list.push(Token {
                        token_type: Lexeme::BinaryOperator(symbol.to_string()),
                        line_number: 0,
                        char_start: symbol_position,
                        char_end: self.position(),
                    });
                },
                // Lambda
                '\\' | '^' => {
                    self.next();

                    token_list.push(Token {
                        token_type: Lexeme::Lambda,
                        line_number: 0,
                        char_start: symbol_position,
                        char_end: self.position(),
                    });
                },
                // Dot
                '.' => {
                    self.next();

                    token_list.push(Token {
                        token_type: Lexeme::Dot,
                        line_number: 0,
                        char_start: symbol_position,
                        char_end: self.position(),
                    });
                },
                // Left Paren
                '(' => {
                    self.next();

                    token_list.push(Token {
                        token_type: Lexeme::LeftParen,
                        line_number: 0,
                        char_start: symbol_position,
                        char_end: self.position(),
                    });
                },
                // Right Paren
                ')' => {
                    self.next();

                    token_list.push(Token {
                        token_type: Lexeme::RightParen,
                        line_number: 0,
                        char_start: symbol_position,
                        char_end: self.position(),
                    });
                },
                // Comma
                ',' => {
                    self.next();

                    token_list.push(Token {
                        token_type: Lexeme::Comma,
                        line_number: 0,
                        char_start: symbol_position,
                        char_end: self.position(),
                    });
                },
                // Equals
                '=' => {
                    self.next();

                    token_list.push(Token {
                        token_type: Lexeme::Equals,
                        line_number: 0,
                        char_start: symbol_position,
                        char_end: self.position(),
                    });
                },
                // End of input
                '\0' => {
                    return Ok(token_list);
                },
                _ => {
                    return Err("Unexpected symbol encountered!".to_string());
                }
            }

            if self.position >= self.input.len() {
                break;
            }
        }

        Ok(token_list)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lexeme
{
    Identifier(String),
    Integer(i64),
    BinaryOperator(String),
    Let,
    In,
    Equals,
    Lambda,
    Dot,
    LeftParen,
    RightParen,
    Comma,
}

pub fn lexeme_from_string(input: String) -> Lexeme {
    match input.as_str() {
        "+" | "-" | "*" | "/" | "%" => Lexeme::BinaryOperator(input),
        "\\" | "^"=> Lexeme::Lambda,
        "." => Lexeme::Dot,
        "(" => Lexeme::LeftParen,
        ")" => Lexeme::RightParen,
        "," => Lexeme::Comma,
        "let" => Lexeme::Let,
        "in" => Lexeme::In,
        "=" => Lexeme::Equals,
        _ => {
            if input.chars().all(char::is_numeric) {
                Lexeme::Integer(input.parse::<i64>().unwrap())
            } else if input.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                Lexeme::Identifier(input)
            } else {
                panic!("Invalid lexeme")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: Lexeme,
    pub line_number: usize,
    pub char_start: usize,
    pub char_end: usize,
}