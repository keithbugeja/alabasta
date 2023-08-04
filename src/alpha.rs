use std::{
    collections::HashMap, 
    sync::atomic::{
        AtomicUsize, 
        Ordering
    }
};

use crate::{ast::{
    ExpressionNode, 
    VariableNode, 
    ConstantNode, 
    AbstractionNode, 
    ApplicationNode, 
    ArithmeticNode, 
    LetNode,
    SyntaxTreeVisitor
}, beta::NormalExpressionNode};

pub struct AlphaConverter {
    variable_scope_stack: HashMap<String, Vec<String>>,
    variable_index: AtomicUsize,
}

impl AlphaConverter {
    pub fn new() -> AlphaConverter {
        AlphaConverter {
            variable_scope_stack: HashMap::new(),
            variable_index: AtomicUsize::new(0),
        }
    }

    pub fn rename(&mut self, name: &String) -> String {
        let scope = self.variable_scope_stack.get(name).unwrap();

        scope.last().unwrap().clone()
    }

    pub fn is_bound(&self, name: &String) -> bool {
        self.variable_scope_stack.contains_key(name) && self.variable_scope_stack.get(name).unwrap().len() > 0
    }

    pub fn bind(&mut self, name: &String) {
        let variable = self.generate();
        
        let scope = self.variable_scope_stack.entry(name.clone()).or_insert(Vec::new());

        scope.push(variable);
    }

    pub fn release(&mut self, name: &String) {
        let scope = self.variable_scope_stack.get_mut(name).unwrap();

        scope.pop();
    }

    pub fn generate(&mut self) -> String {
        let index = self.variable_index.fetch_add(1, Ordering::SeqCst);

        format!("@x{}", index)
    }

    pub fn convert(&mut self, expression: &ExpressionNode) -> bool {
        expression.accept(self);

        true
    }
}

impl SyntaxTreeVisitor for AlphaConverter {
    fn visit_variable(&mut self, node: &VariableNode) {
        if self.is_bound(&node.name.borrow()) {
            let name = self.rename(&node.name.borrow());

            node.name.replace(name);
        }
    }

    fn visit_constant(&mut self, _node: &ConstantNode) { }

    fn visit_expression(&mut self, node: &ExpressionNode) {
        match node {
            ExpressionNode::Variable(node) => {
                self.visit_variable(node);
            },
            ExpressionNode::Constant(node) => {
                self.visit_constant(node);
            },
            ExpressionNode::Abstraction(node) => {
                self.visit_abstraction(node);
            },
            ExpressionNode::Application(node) => {
                self.visit_application(node);
            },
            ExpressionNode::Arithmetic(node) => {
                self.visit_arithmetic(node);
            },
            ExpressionNode::Let(node) => {
                self.visit_let(node);
            },
        }
    }

    fn visit_abstraction(&mut self, node: &AbstractionNode) {
        let variable_name = node.variable.name.borrow().clone();
        
        self.bind(&variable_name);

        node.variable.accept(self);
        node.expression.accept(self);

        self.release(&variable_name);
    }

    fn visit_application(&mut self, node: &ApplicationNode) {
        node.function.accept(self);
        node.argument.accept(self);
    }

    fn visit_arithmetic(&mut self, node: &ArithmeticNode) {
        node.left.accept(self);
        node.right.accept(self);
    }

    fn visit_let(&mut self, node: &LetNode) {
        // node.expression_lhs.accept(self);
        // node.expression_rhs.accept(self);
        node.variable.accept(self);
        node.expression.accept(self);
        node.scope.accept(self);
    }
}