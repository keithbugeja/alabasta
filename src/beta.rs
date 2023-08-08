use std::rc::Rc;

use crate::{
    ast::ExpressionNode, 
    // pretty::{
    //     pretty_print_normal, 
    //     pretty_print
    // }
};

#[derive(Debug, PartialEq, Clone)]
pub enum NormalExpressionNode {
    Variable(String),
    Constant(i64),
    Abstraction(String, Rc<NormalExpressionNode>),
    Application(Rc<NormalExpressionNode>, Rc<NormalExpressionNode>),
    Arithmetic(Rc<NormalExpressionNode>, String, Rc<NormalExpressionNode>),
    Let(String, Rc<NormalExpressionNode>, Rc<NormalExpressionNode>),
}

pub struct BetaReducer {
}

impl BetaReducer {
    pub fn new() -> BetaReducer {
        BetaReducer {
        }
    }

    fn substitute(&self, expression: &NormalExpressionNode, variable: &String, argument: &NormalExpressionNode ) -> Option<NormalExpressionNode> {
        match expression {
            NormalExpressionNode::Constant(_) => {
                Some(expression.clone())
            },
            NormalExpressionNode::Variable(name) => {
                if name == variable {
                    Some(argument.clone())
                } else {
                    Some(NormalExpressionNode::Variable(name.clone()))
                }
            },
            NormalExpressionNode::Abstraction(name, body) => {
                // No substitution: another variable with the same name is bound in this abstraction
                if name == variable {
                    Some(NormalExpressionNode::Abstraction(name.clone(), body.clone()))
                } else {
                    self.substitute(body.as_ref(), variable, argument)
                        .map_or(
                            None, 
                            |body| Some(NormalExpressionNode::Abstraction(name.clone(), Rc::new(body))))
                }
            },
            NormalExpressionNode::Application(function, application) => {
                // Substitute both the function and the application
                let function = self.substitute(function.as_ref(), variable, argument);
                let application = self.substitute(application.as_ref(), variable, argument);

                // If either substitution failed, we can't substitute the application
                let result = match (function, application) {
                    (Some(function), Some(application)) => {
                        Some(NormalExpressionNode::Application(Rc::new(function), Rc::new(application)))
                    },
                    _ => {
                        None
                    }
                };

                // println!("Substitution: Application => "); pretty_print_normal(&result.clone().unwrap()); println!();

                result
            },
            NormalExpressionNode::Arithmetic(left, operator, right) => {
                // Substitute both the left and right side of the arithmetic expression
                let left = self.substitute(left.as_ref(), variable, argument);
                let right = self.substitute(right.as_ref(), variable, argument);

                // If either substitution failed, we can't substitute the arithmetic expression
                let result = match (left, right) {
                    (Some(left), Some(right)) => {
                        Some(NormalExpressionNode::Arithmetic(Rc::new(left), operator.clone(), Rc::new(right)))
                    },
                    _ => {
                        None
                    }
                };

                // println!("Substitution: Arithmetic => "); pretty_print_normal(&result.clone().unwrap()); println!();

                result
            },
            _ => {
                // Technically, we shouldn't be here
                // println!("Substitution: Default");
                Some(NormalExpressionNode::Application(Rc::new(expression.clone()), Rc::new(argument.clone())))
            }
        }
    }

    ///
    /// reduce
    /// 
    pub fn reduce(&mut self, node: &NormalExpressionNode) -> NormalExpressionNode {
        match node {
            // Variables and constants are already in normal form
            NormalExpressionNode::Variable(_) => {
                node.clone()
            },
            NormalExpressionNode::Constant(_) => {
                node.clone()
            },
            // Make sure abstraction body is in NF
            NormalExpressionNode::Abstraction(parameter, body) => {
                let result = NormalExpressionNode::Abstraction(
                    parameter.clone(), 
                    Rc::new(self.reduce(body.as_ref()))
                );

                // print!("Beta Reduction: Reduced body of abstraction ({}) => ", parameter);
                // pretty_print_normal(&result.clone()); println!();

                result
            },
            // Application
            NormalExpressionNode::Application(function, argument) => {                
                // Before applying the function, we need to reduce the function and the argument
                let function = self.reduce(function.as_ref());
                let argument = self.reduce(argument.as_ref());

                // If the function is an abstraction, we need to substitute the parameter with the argument
                let result = match function.clone() {
                    NormalExpressionNode::Abstraction(parameter, body) => 
                    {
                        self.substitute(body.as_ref(), &parameter, &argument)
                            .map_or(
                                NormalExpressionNode::Application(Rc::new(function), Rc::new(argument)), 
                                |body| self.reduce(&body))
                    },
                    _ => {
                        NormalExpressionNode::Application(Rc::new(function), Rc::new(argument))
                    }
                };

                // println!("Beta Reduction: Reduced application => "); pretty_print_normal(&result.clone()); println!();

                result
            },
            // Arithmetic
            NormalExpressionNode::Arithmetic(lhs, operator, rhs) => {
                let lhs = self.reduce(lhs.as_ref());
                let rhs = self.reduce(rhs.as_ref());

                match (lhs.clone(), rhs.clone()) {
                    (NormalExpressionNode::Constant(lhs), NormalExpressionNode::Constant(rhs)) => {
                        match operator.as_str() {
                            "+" => {
                                NormalExpressionNode::Constant(lhs + rhs)
                            },
                            "-" => {
                                NormalExpressionNode::Constant(lhs - rhs)
                            },
                            "*" => {
                                NormalExpressionNode::Constant(lhs * rhs)
                            },
                            "/" => {
                                NormalExpressionNode::Constant(lhs / rhs)
                            },
                            "%" => {
                                NormalExpressionNode::Constant(lhs % rhs)
                            },
                            _ => {
                                NormalExpressionNode::Arithmetic(
                                    Rc::new(NormalExpressionNode::Constant(lhs)), 
                                    operator.clone(), 
                                    Rc::new(NormalExpressionNode::Constant(rhs)))
                            }
                        }
                    },
                    _ => {
                        NormalExpressionNode::Arithmetic(Rc::new(lhs), operator.clone(), Rc::new(rhs))
                    }
                }
            },
            NormalExpressionNode::Let(parameter, expression, body) => {
                let expression = self.reduce(expression.as_ref());
                let body = self.reduce(body.as_ref());

                let result = self.substitute(&body, parameter, &expression)
                    .map_or(
                        NormalExpressionNode::Let(parameter.clone(), Rc::new(expression), Rc::new(body)), 
                        |body| self.reduce(&body));

                // print!("Beta Reduction: Reduced let expression => "); pretty_print_normal(&result.clone()); println!();

                result
            }
        }
    }
}