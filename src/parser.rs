///
/// Parsing
/// 
/// The parser takes a list of tokens and converts them into an abstract syntax tree. Lambda expressions
/// are left-associative with respect to applications (function calls). This means that the expression
/// "x y z" is parsed as "(x y) z".
/// 
/// The language is described by the following EBNF:
/// 
/// Expression  :=  Variable
///             |   Constant
///             |   '\' Variable '.' Expression
///             |   '(' Expression ')'
///             |   Expression Expression
///             |   Expression BinaryOperator Expression
///             |   'let' Variable '=' Expression 'in' Expression
/// 
/// Variable    :=  Identifier
/// 
/// Constant    :=  Integer
/// 
/// BinaryOperator := '+' | '-' | '*' | '/'
/// 
/// Identifier  :=  [a-zA-Z]+
/// 
/// Integer     :=  [0-9]+
///  
use crate::lexer::{Token, Lexeme};
use crate::ast::{ExpressionNode, VariableNode, ConstantNode, AbstractionNode, ApplicationNode, ArithmeticNode, LetNode};

use std::cell::RefCell;
use std::rc::Rc;

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
            (_, _) => { 
                None 
            },
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
        //      | let I = E in E

        let mut left = self.parse_single_expression()?;

        loop {
            if let Some(right) = self.parse_single_expression() {
                left = ExpressionNode::Application(
                    ApplicationNode {
                        function: Rc::new(left),
                        argument: Rc::new(right),
                    }                
                );
            } else if let Some(arithmetic) = self.parse_binary_operation(left.clone()) {
                left = arithmetic;
            } else {
                break;
            }
        }
    
        Some(left)
    }

    fn parse_let_expression(&mut self) -> Option<ExpressionNode> {
        let _ = self.expect(Lexeme::Let)?;
        
        let variable = match self.parse_variable()? {
            ExpressionNode::Variable(variable) => { 
                variable
            },
            _ => { return None; }
        };
        
        let _ = self.expect(Lexeme::Equals)?;
        let expression  = self.parse_expression()?;

        let _ = self.expect(Lexeme::In)?;
        let scope = self.parse_expression()?;

        Some(ExpressionNode::Let(
            LetNode {
                variable: variable,
                expression: Rc::new(expression),
                scope: Rc::new(scope),
            }
        ))
    }

    fn parse_single_expression(&mut self) -> Option<ExpressionNode> {        
        let token = self.peek()?;

        let expression = match token.token_type {
            Lexeme::Lambda => self.parse_abstraction(),
            Lexeme::LeftParen => self.parse_subexpression(),
            Lexeme::Let => self.parse_let_expression(),
            Lexeme::Identifier(_) | Lexeme::Integer (_) => self.parse_arithmetic(),
            _ => { None },
        };

        expression
    }

    fn parse_subexpression(&mut self) -> Option<ExpressionNode> {
        let _ = self.expect(Lexeme::LeftParen)?;
        let expression = self.parse_expression()?;
        let _ = self.expect(Lexeme::RightParen)?;

        Some(expression)
    }

    fn parse_abstraction(&mut self) -> Option<ExpressionNode> {
        let _ = self.expect(Lexeme::Lambda)?;

        let variable = match self.parse_variable()? {
            ExpressionNode::Variable(variable) => { 
                variable
            },
            _ => { return None; }
        };
        
        let _ = self.expect(Lexeme::Dot)?;
        let expression = self.parse_expression()?;
    
        return Some(ExpressionNode::Abstraction(
            AbstractionNode {
                variable: Rc::new(variable),
                expression: Rc::new(expression),
            }
        ));
    }
    
    fn parse_binary_operation(&mut self, left: ExpressionNode) -> Option<ExpressionNode> {
        let operator = self.expect(Lexeme::BinaryOperator(String::new()))?;

        if let Lexeme::BinaryOperator(value) = operator.token_type {
            let right = self.parse_single_expression()?;
            let expression = ExpressionNode::Arithmetic(ArithmeticNode {
                operator: value,
                left: Rc::new(left),
                right: Rc::new(right),
            });
            Some(expression)
        } else {
            None
        }
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
                Some(ExpressionNode::Variable(VariableNode { name: Rc::new(RefCell::new(value))}))
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
                Some(ExpressionNode::Constant(ConstantNode { value }))
            } else {
                None
            }
        } else {
            None
        }
    }
}
