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

    // ///
    // /// convert
    // /// 
    // pub fn convert(&mut self, node: &ExpressionNode) -> NormalExpressionNode {
    //     match node {
    //         ExpressionNode::Variable(node) => {
    //             NormalExpressionNode::Variable(node.name.borrow().clone())
    //         },
    //         ExpressionNode::Constant(node) => {
    //             NormalExpressionNode::Constant(node.value)
    //         },
    //         ExpressionNode::Abstraction(node) => {
    //             let body = self.convert(node.expression.as_ref());
    //             let parameter = node.variable.name.borrow().clone();                
    //             NormalExpressionNode::Abstraction(parameter, Rc::new(body))
    //         },
    //         ExpressionNode::Application(node) => {
    //             let function = self.convert(node.function.as_ref());
    //             let argument = self.convert(node.argument.as_ref());
    //             NormalExpressionNode::Application(Rc::new(function), Rc::new(argument))
    //         },
    //         ExpressionNode::Arithmetic(node) => {
    //             let lhs = self.convert(node.left.as_ref());
    //             let rhs = self.convert(node.right.as_ref());

    //             let result = match (lhs, rhs) {
    //                 (NormalExpressionNode::Constant(a), NormalExpressionNode::Constant(b)) => { 
    //                     match node.operator.as_str() {
    //                         "+" => NormalExpressionNode::Constant(a + b),
    //                         "-" => NormalExpressionNode::Constant(a - b),
    //                         "*" => NormalExpressionNode::Constant(a * b),
    //                         "/" => NormalExpressionNode::Constant(a / b),
    //                         "%" => NormalExpressionNode::Constant(a % b),
    //                         _ => panic!("Unknown operator"),
    //                     }
    //                 }
    //                 (_, _) => { 
    //                     let lhs = self.convert(node.left.as_ref());
    //                     let rhs = self.convert(node.right.as_ref());
    //                     let operator = node.operator.clone();
                        
    //                     NormalExpressionNode::Arithmetic(Rc::new(lhs), operator, Rc::new(rhs))
    //                 }
    //             };

    //             result
    //         },
    //         // let var = expr in expr
    //         ExpressionNode::Let(node) => {
    //             let parameter = node.variable.name.borrow().clone();
    //             let expression = self.convert(node.expression.as_ref());
    //             let body = self.convert(node.scope.as_ref());

    //             NormalExpressionNode::Let(parameter, Rc::new(expression), Rc::new(body))
    //         },
    //     }        
    // }
}