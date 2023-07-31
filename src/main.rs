mod lexer;
mod parser;
mod ast;

use lexer::Lexer;
use parser::Parser;
use ast::{
    ExpressionNode, 
    VariableNode, 
    ConstantNode, 
    AbstractionNode, 
    ApplicationNode, 
    ArithmeticNode, 
    LetNode
};

fn main() {
    //let expression = "x y z";
    // let expression = r"(\f. \x. f (f x)) (\y. y * 2) 3";
    let expression = r"(\x. x + 3) ((\y. y * 2) 5) + (\z. z / 2) (4 - 1)";
    // let expression = r"let double = \x. x * 2 in double 5";

    // let expression = r"let f x = g x in
    //                    let g x = f x in
    //                    let x = 1 in
    //                    f (g (f (g x)))";

    //let expression = r"(\f. x)";
    
    let mut lexer = Lexer::new(expression.to_string());
    let token_list = lexer.scan().unwrap();
    //println!("{:?}", token_list);

    let ast = Parser::new(token_list).parse().unwrap();

    alpha_conversion(ast.clone()).unwrap();

    // println!("\n{:?}", ast);
}

///
/// Evaluation
///
/// alpha-equivalence, alpha-conversion and beta-reduction
/// 
/// alpha-equivalence: Two lambda expressions are alpha-equivalent if they differ only in the names of their bound variables.
/// alpha-conversion: The process of converting a lambda expression to alpha-equivalent form.
/// beta-reduction: The process of applying a lambda expression to an argument.
///
/// The following rules are used to perform beta-reduction:
/// (\x. E) V -> E[V/x]
///
/// The following rules are used to perform alpha-conversion:
/// (\x. E) -> (\y. E[y/x]) where y is not free in E
/// 

pub fn alpha_conversion(node: ExpressionNode) -> Result<ExpressionNode, ()> {
    // loop {
        match &node { 
            ExpressionNode::Abstraction(abstraction) => {
                let parameter = abstraction.parameter.clone();
                println!("parameter: {:?}", parameter);
            },
            _ => {println!("{:?}", node)},
        }
    // }

    Ok(node)
}

pub fn beta_conversion(node: ExpressionNode) -> Result<ExpressionNode, ()> {
    Ok(node)
}

// #[derive(Debug, PartialEq, Clone)]
// pub enum ExpressionNode {
//     Variable(VariableNode),
//     Constant(ConstantNode),
//     Abstraction(AbstractionNode),
//     Application(ApplicationNode),
//     Arithmetic(ArithmeticNode),
//     Let(LetNode),
//     SubExpression(Rc<ExpressionNode>),    
// }

// #[derive(Debug, PartialEq, Clone)]
// pub struct VariableNode {
//     name: String,
// }

// #[derive(Debug, PartialEq, Clone)]
// pub struct ConstantNode {
//     value: i64,
// }

// #[derive(Debug, PartialEq, Clone)]
// pub struct AbstractionNode {
//     parameter: Rc<VariableNode>,
//     body: Rc<ExpressionNode>,
// }

// #[derive(Debug, PartialEq, Clone)]
// pub struct ApplicationNode {
//     function: Rc<ExpressionNode>,
//     argument: Rc<ExpressionNode>,    
// }

// #[derive(Debug, PartialEq, Clone)]
// pub struct ArithmeticNode {
//     operator: String,
//     left: Rc<ExpressionNode>,
//     right: Rc<ExpressionNode>,
// }


// #[derive(Debug, PartialEq, Clone)]
// pub struct LetNode {
//     expression_lhs: Rc<ExpressionNode>,
//     expression_rhs: Rc<ExpressionNode>,
//     body: Rc<ExpressionNode>,
// }

// ///
// /// Parsing
// /// 
// /// The parser takes a list of tokens and converts them into an abstract syntax tree. Lambda expressions
// /// are left-associative with respect to applications (function calls). This means that the expression
// /// "x y z" is parsed as "(x y) z".
// /// 
// /// The language is described by the following EBNF:
// /// 
// /// Expression  :=  Variable
// ///             |   Constant
// ///             |   '\' Variable '.' Expression
// ///             |   '(' Expression ')'
// ///             |   Expression Expression
// ///             |   Expression BinaryOperator Expression
// ///             |   'let' Variable '=' Expression 'in' Expression
// /// 
// /// Variable    :=  Identifier
// /// 
// /// Constant    :=  Integer
// /// 
// /// BinaryOperator := '+' | '-' | '*' | '/'
// /// 
// /// Identifier  :=  [a-zA-Z]+
// /// 
// /// Integer     :=  [0-9]+
// ///  

// pub struct Parser {
//     token_list: Vec::<Token>,
//     position: usize,
// }

// impl Parser {
//     pub fn new(token_list: Vec::<Token>) -> Parser {
//         Parser {
//             token_list,
//             position: 0,
//         }
//     }

//     fn position(&self) -> usize {
//         self.position
//     }

//     fn next(&mut self) -> Option<Token> {
//         let token = self.peek();
        
//         self.position += 1;

//         token
//     }

//     fn peek(&self) -> Option<Token> {
//         if self.position >= self.token_list.len() {
//             return None;
//         }

//         if let Some(token) = self.token_list.get(self.position) {
//             return Some(token.clone());
//         }

//         return None;
//     }

//     fn expect(&mut self, token_kind: Lexeme) -> Option<Token> {
//         let token = self.peek()?.token_type.clone();

//         match (token, token_kind) {
//             (Lexeme::Identifier(_), Lexeme::Identifier(_)) => {
//                 self.next()
//             },
//             (Lexeme::Integer(_), Lexeme::Integer(_)) => {
//                 self.next()
//             },
//             (Lexeme::BinaryOperator(_), Lexeme::BinaryOperator(_)) => {
//                 self.next()
//             },
//             (one, two) if one == two => {
//                 self.next()
//             },
//             (_, _) => { 
//                 None 
//             },
//         }
//     }

//     pub fn parse(&mut self) -> Result<ExpressionNode, ()> {
//         self.parse_expression().map_or(Err(()), |node| Ok(node))
//     }

//     fn parse_expression(&mut self) -> Option<ExpressionNode> {
//         // --------------------
//         // EBNF
//         // --------------------
//         // E :=
//         //      | \I. E
//         //      | E E
//         //      | E <op> E
//         //      | (E)
//         //      | I | C

//         let mut left = self.parse_single_expression()?;

//         loop {
//             if let Some(right) = self.parse_single_expression() {
//                 left = ExpressionNode::Application(
//                     ApplicationNode {
//                         function: Rc::new(left),
//                         argument: Rc::new(right),
//                     }                
//                 );
//             } else if let Some(arithmetic) = self.parse_binary_operation(left.clone()) {
//                 left = arithmetic;
//             } else {
//                 break;
//             }
//         }
    
//         Some(left)
//     }

//     fn parse_let_expression(&mut self) -> Option<ExpressionNode> {
//         print!("let ");
        
//         let _ = self.expect(Lexeme::Let)?;
        
//         // let variable = match self.parse_variable()? {
//         //     ExpressionNode::Variable(variable) => { 
//         //         variable
//         //     },
//         //     _ => { return None; }
//         // };

//         let expression_lhs = self.parse_expression()?;

//         print!(" = ");
        
//         let _ = self.expect(Lexeme::Equals)?;
//         let expression_rhs  = self.parse_expression()?;
        
//         print!(" in ");

//         let _ = self.expect(Lexeme::In)?;
//         let body = self.parse_expression()?;

//         Some(ExpressionNode::Let(
//             LetNode {
//                 expression_lhs: Rc::new(expression_lhs),
//                 expression_rhs: Rc::new(expression_rhs),
//                 body: Rc::new(body),
//             }
//         ))
//     }

//     fn parse_single_expression(&mut self) -> Option<ExpressionNode> {        
//         let token = self.peek()?;

//         let expression = match token.token_type {
//             Lexeme::Lambda => self.parse_abstraction(),
//             Lexeme::LeftParen => self.parse_subexpression(),
//             Lexeme::Let => self.parse_let_expression(),
//             Lexeme::Identifier(_) | Lexeme::Integer (_) => self.parse_arithmetic(),
//             _ => { None },
//         };

//         expression
//     }

//     fn parse_subexpression(&mut self) -> Option<ExpressionNode> {
//         print!("(");
//         let _ = self.expect(Lexeme::LeftParen)?;
//         let expression = self.parse_expression()?;
//         let _ = self.expect(Lexeme::RightParen)?;
//         print!(")");

//         Some(expression)
//     }

//     fn parse_abstraction(&mut self) -> Option<ExpressionNode> {
//         print!("λ");
//         let _ = self.expect(Lexeme::Lambda)?;

//         let variable = match self.parse_variable()? {
//             ExpressionNode::Variable(variable) => { 
//                 variable
//             },
//             _ => { return None; }
//         };
        
//         let _ = self.expect(Lexeme::Dot)?;
//         print!(".");
        
//         let expression = self.parse_expression()?;
    
//         return Some(ExpressionNode::Abstraction(
//             AbstractionNode {
//                 parameter: Rc::new(variable),
//                 body: Rc::new(expression),
//             }
//         ));
//     }
    
//     fn parse_binary_operation(&mut self, left: ExpressionNode) -> Option<ExpressionNode> {
//         let operator = self.expect(Lexeme::BinaryOperator(String::new()))?;

//         if let Lexeme::BinaryOperator(value) = operator.token_type {
//             print!(" {} ", value);

//             let right = self.parse_single_expression()?;
//             let expression = ExpressionNode::Arithmetic(ArithmeticNode {
//                 operator: value,
//                 left: Rc::new(left),
//                 right: Rc::new(right),
//             });
//             Some(expression)
//         } else {
//             None
//         }
//     }

//     fn parse_arithmetic(&mut self) -> Option<ExpressionNode> {
//         let left = match self.peek()?.token_type {
//             Lexeme::Identifier(_) => self.parse_variable()?,
//             Lexeme::Integer(_) => self.parse_constant()?,
//             _ => { return None; }
//         };

//         if let Some(operator) = self.expect(Lexeme::BinaryOperator(String::new())) {
//             match operator.token_type {
//                 Lexeme::BinaryOperator(value) => {
//                     print!(" {} ", value);
                    
//                     let right = match self.peek()?.token_type {
//                         Lexeme::Identifier(_) => self.parse_variable()?,
//                         Lexeme::Integer(_) => self.parse_constant()?,
//                         _ => { return None }
//                     };

//                     return Some(ExpressionNode::Arithmetic(
//                         ArithmeticNode {
//                             operator: value,
//                             left: Rc::new(left),
//                             right: Rc::new(right),
//                         }
//                     ));
//                 }
//                 _ => { return None; }
//             }
//         } 
        
//         Some(left)
//     }

//     fn parse_variable(&mut self) -> Option<ExpressionNode> {
//         if let Some(identifier) = self.expect(Lexeme::Identifier(String::new())) {
//             if let Lexeme::Identifier(value) = identifier.token_type {
//                 print!("{}", value);
//                 Some(ExpressionNode::Variable(VariableNode { name: value }))
//             } else {
//                 None
//             }
//         } else {
//             None
//         }
//     }

//     fn parse_constant(&mut self) -> Option<ExpressionNode> {
//         if let Some(integer) = self.expect(Lexeme::Integer(0)) {
//             if let Lexeme::Integer(value) = integer.token_type {
//                 print!("{}", value);
//                 Some(ExpressionNode::Constant(ConstantNode { value }))
//             } else {
//                 None
//             }
//         } else {
//             None
//         }
//     }
// }

// ///
// /// Lexical analysis (tokenization)
// /// 
// /// The lexer takes a string of characters and converts it into a list of tokens.
// /// 

// #[derive(Debug, PartialEq)]
// pub struct Lexer {
//     input: String,
//     position: usize,
// }

// impl Lexer {
//     pub fn new(input: String) -> Lexer {
//         Lexer {
//             input,
//             position: 0,
//         }
//     }

//     fn position(&self) -> usize {
//         self.position
//     }

//     fn next(&mut self) -> Option<char> {
//         let chr = self.peek();
        
//         self.position += 1;

//         chr
//     }

//     fn peek(&mut self) -> Option<char> {
//         if self.position >= self.input.len() {
//             return None;
//         }

//         if let Some(chr_as_str) = self.input.get(self.position..self.position+1) {
//             return chr_as_str.chars().next();
//         }

//         return None;
//     }

//     pub fn scan(&mut self) -> Result<Vec::<Token>, ()> {
//         let mut symbol;
//         let mut symbol_position;
//         let mut token_list = Vec::<Token>::new();

//         loop {
//             symbol = self.peek().unwrap();
//             symbol_position = self.position;

//             match symbol
//             {
//                 // Whitespace
//                 ' ' | '\t' | '\n' => {
//                     self.next();
//                 },
//                 // Identifier
//                 'a'..='z' | 'A'..='Z' => {
//                     let mut identifier = String::new();

//                     while let Some(chr) = self.peek() {
//                         if chr.is_alphanumeric() {
//                             identifier.push(chr);
//                             self.next();
//                         } else {
//                             break;
//                         }
//                     }

//                     token_list.push(Token {
//                         token_type: lexeme_from_string(identifier),
//                         line_number: 0,
//                         char_start: symbol_position,
//                         char_end: self.position(),
//                     });
//                 },
//                 // Integer
//                 '0'..='9' => {
//                     let mut integer = String::new();

//                     while let Some(chr) = self.peek() {
//                         if chr.is_numeric() {
//                             integer.push(chr);
//                             self.next();
//                         } else {
//                             break;
//                         }
//                     }

//                     token_list.push(Token {
//                         token_type: Lexeme::Integer(integer.parse::<i64>().unwrap()),
//                         line_number: 0,
//                         char_start: symbol_position,
//                         char_end: self.position(),
//                     });
//                 },
//                 // Binary Operator
//                 '+' | '-' | '*' | '/' | '%' => {                    
//                     self.next();

//                     token_list.push(Token {
//                         token_type: Lexeme::BinaryOperator(symbol.to_string()),
//                         line_number: 0,
//                         char_start: symbol_position,
//                         char_end: self.position(),
//                     });
//                 },
//                 // Lambda
//                 '\\' => {
//                     self.next();

//                     token_list.push(Token {
//                         token_type: Lexeme::Lambda,
//                         line_number: 0,
//                         char_start: symbol_position,
//                         char_end: self.position(),
//                     });
//                 },
//                 // Dot
//                 '.' => {
//                     self.next();

//                     token_list.push(Token {
//                         token_type: Lexeme::Dot,
//                         line_number: 0,
//                         char_start: symbol_position,
//                         char_end: self.position(),
//                     });
//                 },
//                 // Left Paren
//                 '(' => {
//                     self.next();

//                     token_list.push(Token {
//                         token_type: Lexeme::LeftParen,
//                         line_number: 0,
//                         char_start: symbol_position,
//                         char_end: self.position(),
//                     });
//                 },
//                 // Right Paren
//                 ')' => {
//                     self.next();

//                     token_list.push(Token {
//                         token_type: Lexeme::RightParen,
//                         line_number: 0,
//                         char_start: symbol_position,
//                         char_end: self.position(),
//                     });
//                 },
//                 // Comma
//                 ',' => {
//                     self.next();

//                     token_list.push(Token {
//                         token_type: Lexeme::Comma,
//                         line_number: 0,
//                         char_start: symbol_position,
//                         char_end: self.position(),
//                     });
//                 },
//                 // Equals
//                 '=' => {
//                     self.next();

//                     token_list.push(Token {
//                         token_type: Lexeme::Equals,
//                         line_number: 0,
//                         char_start: symbol_position,
//                         char_end: self.position(),
//                     });
//                 },
//                 // End of input
//                 '\0' => {
//                     return Ok(token_list);
//                 },
//                 _ => {
//                     return Err(());
//                 }
//             }

//             if self.position >= self.input.len() {
//                 break;
//             }
//         }

//         Ok(token_list)
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum Lexeme
// {
//     Identifier(String),
//     Integer(i64),
//     BinaryOperator(String),
//     Let,
//     In,
//     Equals,
//     Lambda,
//     Dot,
//     LeftParen,
//     RightParen,
//     Comma,
// }

// pub fn lexeme_from_string(input: String) -> Lexeme {
//     match input.as_str() {
//         "+" | "-" | "*" | "/" | "%" => Lexeme::BinaryOperator(input),
//         "\\" => Lexeme::Lambda,
//         "." => Lexeme::Dot,
//         "(" => Lexeme::LeftParen,
//         ")" => Lexeme::RightParen,
//         "," => Lexeme::Comma,
//         "let" => Lexeme::Let,
//         "in" => Lexeme::In,
//         "=" => Lexeme::Equals,
//         _ => {
//             if input.chars().all(char::is_numeric) {
//                 Lexeme::Integer(input.parse::<i64>().unwrap())
//             } else if input.chars().all(char::is_alphabetic) {
//                 Lexeme::Identifier(input)
//             } else {
//                 panic!("Invalid lexeme")
//             }
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct Token {
//     token_type: Lexeme,
//     line_number: usize,
//     char_start: usize,
//     char_end: usize,
// }