use crate::ast::{
    ExpressionNode, 
    VariableNode, 
    ConstantNode, 
    AbstractionNode, 
    ApplicationNode, 
    ArithmeticNode, 
    LetNode,
    SyntaxTreeVisitor
};

use crate::beta::NormalExpressionNode;

pub fn pretty_print(node: &ExpressionNode) {
    match node {
        ExpressionNode::Variable(node) => {
            print!("{}", node.name.borrow());
        },
        ExpressionNode::Constant(node) => {
            print!("{}", node.value);
        },
        ExpressionNode::Abstraction(node) => {
            print!("(λ{}. ", node.variable.as_ref().name.borrow().clone());
            pretty_print(node.expression.as_ref());
            print!(")");
        },
        ExpressionNode::Application(node) => {
            print!("(");
            pretty_print(node.function.as_ref());
            print!(" ");
            pretty_print(node.argument.as_ref());
            print!(")");
        },
        ExpressionNode::Arithmetic(node) => {
            print!("(");
            pretty_print(node.left.as_ref());
            print!(" {} ", node.operator);
            pretty_print(node.right.as_ref());
            print!(")");
        },
        ExpressionNode::Let(node) => {
            print!("let {} = ", node.variable.name.borrow().clone());
            pretty_print(node.expression.as_ref());
            print!(" in ");
            pretty_print(node.scope.as_ref());
        },
        _ => { }
    }
}

pub fn pretty_print_normal(node: &NormalExpressionNode) 
{
    match node {
        NormalExpressionNode::Variable(name) => {
            print!("{}", name);
        },
        NormalExpressionNode::Constant(value) => {
            print!("{}", value);
        },
        NormalExpressionNode::Abstraction(parameter, body) => {
            print!("(λ{}. ", parameter);
            pretty_print_normal(body);
            print!(")");
        },
        NormalExpressionNode::Application(function, argument) => {
            print!("(");
            pretty_print_normal(function);
            print!(" ");
            pretty_print_normal(argument);
            print!(")");
        },
        NormalExpressionNode::Arithmetic(lhs, operator, rhs) => {
            print!("(");
            pretty_print_normal(lhs);
            print!(" {} ", operator);
            pretty_print_normal(rhs);
            print!(")");
        },
        _ => { }
    }
}