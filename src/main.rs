use std::rc::Rc;


fn main() {
    //let expression = "x y z";
    let expression = r"(\f. \x. f (f x)) (\y. y * 2) 3";
    //let expression = r"(\f. x)";
    
    let mut lexer = Lexer::new(expression.to_string());
    let token_list = lexer.scan().unwrap();
    //println!("{:?}", token_list);

    let ast = Parser::new(token_list).parse().unwrap();

    println!("\n{:?}", ast);
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionNode {
    Variable(VariableNode),
    Constant(ConstantNode),
    Abstraction(AbstractionNode),
    Application(ApplicationNode),
    Arithmetic(ArithmeticNode),
    SubExpression(Rc<ExpressionNode>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableNode {
    name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConstantNode {
    value: i64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AbstractionNode {
    parameter: Rc<VariableNode>,
    body: Rc<ExpressionNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ApplicationNode {
    function: Rc<ExpressionNode>,
    argument: Rc<ExpressionNode>,    
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArithmeticNode {
    operator: String,
    left: Rc<ExpressionNode>,
    right: Rc<ExpressionNode>,
}

pub struct Parser {
    token_list: Vec::<Token>,
    position: usize,
}

impl Parser {
    pub fn new(token_list: Vec::<Token>) -> Parser {
        Parser {
            token_list,
            position: 0,
        }
    }

    fn position(&self) -> usize {
        self.position
    }

    fn next(&mut self) -> Option<Token> {
        let token = self.peek();
        
        self.position += 1;

        token
    }

    fn peek(&self) -> Option<Token> {
        if self.position >= self.token_list.len() {
            return None;
        }

        if let Some(token) = self.token_list.get(self.position) {
            return Some(token.clone());
        }

        return None;
    }

    fn expect(&mut self, token_kind: Lexeme) -> Option<Token> {
        let token = self.peek()?.token_type.clone();

        match (token, token_kind) {
            (Lexeme::Identifier(_), Lexeme::Identifier(_)) => {
                self.next()
            },
            (Lexeme::Integer(_), Lexeme::Integer(_)) => {
                self.next()
            },
            (Lexeme::BinaryOperator(_), Lexeme::BinaryOperator(_)) => {
                self.next()
            },
            (one, two) if one == two => {
                self.next()
            },
            (_, _) => { None },
        }
    }

    pub fn parse(&mut self) -> Result<ExpressionNode, ()> {
        self.parse_expression().map_or(Err(()), |node| Ok(node))
    }

    fn parse_expression(&mut self) -> Option<ExpressionNode> {
        // --------------------
        // EBNF
        // --------------------
        // E :=
        //      | \I. E
        //      | E E
        //      | E <op> E
        //      | (E)
        //      | I | C

        let mut left = self.parse_single_expression()?;
    
        while let Some(right) = self.parse_single_expression() {
            left = ExpressionNode::Application(
                ApplicationNode {
                    function: Rc::new(left),
                    argument: Rc::new(right),
                }                
            );
        }
    
        Some(left)
    }

    fn parse_single_expression(&mut self) -> Option<ExpressionNode> {        
        let token = self.peek()?;

        let expression = match token.token_type {
            Lexeme::Lambda => self.parse_abstraction(),
            Lexeme::LeftParen => self.parse_subexpression(),
            Lexeme::Identifier(_) | Lexeme::Integer (_) => self.parse_arithmetic(),
            _ => { None },
        };

        expression
    }

    fn parse_subexpression(&mut self) -> Option<ExpressionNode> {
        print!("(");
        let _ = self.expect(Lexeme::LeftParen)?;
        let expression = self.parse_expression()?;
        let _ = self.expect(Lexeme::RightParen)?;
        print!(")");

        Some(expression)
    }

    fn parse_abstraction(&mut self) -> Option<ExpressionNode> 
    {
        print!("λ");
        let _ = self.expect(Lexeme::Lambda)?;

        let variable = match self.parse_variable()? {
            ExpressionNode::Variable(variable) => { 
                variable
            },
            _ => { return None; }
        };
        
        let _ = self.expect(Lexeme::Dot)?;
        print!(".");
        
        let expression = self.parse_expression()?;
    
        return Some(ExpressionNode::Abstraction(
            AbstractionNode {
                parameter: Rc::new(variable),
                body: Rc::new(expression),
            }
        ));
    }

    fn parse_arithmetic(&mut self) -> Option<ExpressionNode> {
        let left = match self.peek()?.token_type {
            Lexeme::Identifier(_) => self.parse_variable()?,
            Lexeme::Integer(_) => self.parse_constant()?,
            _ => { return None; }
        };

        if let Some(operator) = self.expect(Lexeme::BinaryOperator(String::new())) {
            match operator.token_type {
                Lexeme::BinaryOperator(value) => {
                    print!(" {} ", value);
                    
                    let right = match self.peek()?.token_type {
                        Lexeme::Identifier(_) => self.parse_variable()?,
                        Lexeme::Integer(_) => self.parse_constant()?,
                        _ => { return None }
                    };

                    return Some(ExpressionNode::Arithmetic(
                        ArithmeticNode {
                            operator: value,
                            left: Rc::new(left),
                            right: Rc::new(right),
                        }
                    ));
                }
                _ => { return None; }
            }
        } 
        
        Some(left)
    }

    fn parse_variable(&mut self) -> Option<ExpressionNode> {
        if let Some(identifier) = self.expect(Lexeme::Identifier(String::new())) {
            if let Lexeme::Identifier(value) = identifier.token_type {
                print!("{}", value);
                Some(ExpressionNode::Variable(VariableNode { name: value }))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_constant(&mut self) -> Option<ExpressionNode> {
        if let Some(integer) = self.expect(Lexeme::Integer(0)) {
            if let Lexeme::Integer(value) = integer.token_type {
                print!("{}", value);
                Some(ExpressionNode::Constant(ConstantNode { value }))
            } else {
                None
            }
        } else {
            None
        }
    }
}

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
            return None;
        }

        if let Some(chr_as_str) = self.input.get(self.position..self.position+1) {
            return chr_as_str.chars().next();
        }

        return None;
    }

    pub fn scan(&mut self) -> Result<Vec::<Token>, ()> {
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
                        if chr.is_alphanumeric() {
                            identifier.push(chr);
                            self.next();
                        } else {
                            break;
                        }
                    }

                    token_list.push(Token {
                        token_type: Lexeme::Identifier(identifier),
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
                '\\' => {
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
                // End of input
                '\0' => {
                    return Ok(token_list);
                },
                _ => {
                    return Err(());
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
    Lambda,
    Dot,
    LeftParen,
    RightParen,
    Comma,
}

pub fn lexeme_from_string(input: String) -> Lexeme {
    match input.as_str() {
        "+" | "-" | "*" | "/" | "%" => Lexeme::BinaryOperator(input),
        "\\" => Lexeme::Lambda,
        "." => Lexeme::Dot,
        "(" => Lexeme::LeftParen,
        ")" => Lexeme::RightParen,
        "," => Lexeme::Comma,
        _ => {
            if input.chars().all(char::is_numeric) {
                Lexeme::Integer(input.parse::<i64>().unwrap())
            } else if input.chars().all(char::is_alphabetic) {
                Lexeme::Identifier(input)
            } else {
                panic!("Invalid lexeme")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: Lexeme,
    line_number: usize,
    char_start: usize,
    char_end: usize,
}