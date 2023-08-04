use std::rc::Rc;

use crate::{
    beta::NormalExpressionNode, 
    ast::{
        ExpressionNode, 
        VariableNode, 
        ConstantNode, 
        AbstractionNode, 
        ApplicationNode, 
        ArithmeticNode, 
        LetNode
    }
};

pub fn from_normal_form(node: &NormalExpressionNode) -> ExpressionNode {
    match node {
        NormalExpressionNode::Variable(name) => {
            ExpressionNode::Variable(VariableNode::new(name.as_str()))
        },
        NormalExpressionNode::Constant(value) => {
            ExpressionNode::Constant(ConstantNode{ value: *value })
        },
        NormalExpressionNode::Abstraction(parameter, body) => {
            ExpressionNode::Abstraction(AbstractionNode
                { 
                    variable: Rc::new(VariableNode::new(parameter.as_str())),
                    expression: Rc::new(from_normal_form(body.as_ref()))
                }
            )
        },
        NormalExpressionNode::Application(function, argument) => {
            ExpressionNode::Application(ApplicationNode
                { 
                    function: Rc::new(from_normal_form(function.as_ref())),
                    argument: Rc::new(from_normal_form(argument.as_ref()))
                }
            )
        },
        NormalExpressionNode::Arithmetic(lhs, operator, rhs) => {
            ExpressionNode::Arithmetic(ArithmeticNode
                {
                    operator: operator.clone(),
                    left: Rc::new(from_normal_form(lhs.as_ref())),
                    right: Rc::new(from_normal_form(rhs.as_ref()))
                }
            )
        },
        NormalExpressionNode::Let(variable, expression, scope) => {
            ExpressionNode::Let(LetNode
                {
                    variable: VariableNode::new(variable.as_str()),
                    expression: Rc::new(from_normal_form(expression.as_ref())),
                    scope: Rc::new(from_normal_form(scope.as_ref()))
                }
            )
        },
    }    
}

pub fn to_normal_form(node: &ExpressionNode) -> NormalExpressionNode {
    match node {
        ExpressionNode::Variable(node) => {
            NormalExpressionNode::Variable(node.name.borrow().clone())
        },
        ExpressionNode::Constant(node) => {
            NormalExpressionNode::Constant(node.value)
        },
        ExpressionNode::Abstraction(node) => {
            let body = to_normal_form(node.expression.as_ref());
            let parameter = node.variable.name.borrow().clone();                
            NormalExpressionNode::Abstraction(parameter, Rc::new(body))
        },
        ExpressionNode::Application(node) => {
            let function = to_normal_form(node.function.as_ref());
            let argument = to_normal_form(node.argument.as_ref());
            NormalExpressionNode::Application(Rc::new(function), Rc::new(argument))
        },
        ExpressionNode::Arithmetic(node) => {
            let lhs = to_normal_form(node.left.as_ref());
            let rhs = to_normal_form(node.right.as_ref());

            let result = match (lhs, rhs) {
                (NormalExpressionNode::Constant(a), NormalExpressionNode::Constant(b)) => { 
                    match node.operator.as_str() {
                        "+" => NormalExpressionNode::Constant(a + b),
                        "-" => NormalExpressionNode::Constant(a - b),
                        "*" => NormalExpressionNode::Constant(a * b),
                        "/" => NormalExpressionNode::Constant(a / b),
                        "%" => NormalExpressionNode::Constant(a % b),
                        _ => panic!("Unknown operator"),
                    }
                }
                (_, _) => { 
                    let lhs = to_normal_form(node.left.as_ref());
                    let rhs = to_normal_form(node.right.as_ref());
                    let operator = node.operator.clone();
                    
                    NormalExpressionNode::Arithmetic(Rc::new(lhs), operator, Rc::new(rhs))
                }
            };

            result
        },
        // let var = expr in expr
        ExpressionNode::Let(node) => {
            let parameter = node.variable.name.borrow().clone();
            let expression = to_normal_form(node.expression.as_ref());
            let body = to_normal_form(node.scope.as_ref());

            NormalExpressionNode::Let(parameter, Rc::new(expression), Rc::new(body))
        },
    }        
}