use std::{rc::Rc, cell::RefCell};

///
/// Visitor Pattern trait for abstract syntax tree
/// 
pub trait SyntaxTreeVisitor {
    fn visit_expression(&mut self, node: &ExpressionNode);
    fn visit_variable(&mut self, node: &VariableNode);
    fn visit_constant(&mut self, node: &ConstantNode);
    fn visit_abstraction(&mut self, node: &AbstractionNode);
    fn visit_application(&mut self, node: &ApplicationNode);
    fn visit_arithmetic(&mut self, node: &ArithmeticNode);
    fn visit_let(&mut self, node: &LetNode);
}

///
/// Expression Node
/// 
#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionNode {
    Variable(VariableNode),
    Constant(ConstantNode),
    Abstraction(AbstractionNode),
    Application(ApplicationNode),
    Arithmetic(ArithmeticNode),
    Let(LetNode),
    SubExpression(Rc<ExpressionNode>),
}

impl ExpressionNode {
    pub fn accept(&self, visitor: &mut dyn SyntaxTreeVisitor) {
        visitor.visit_expression(self);
    }
}


///
/// Variable Node
/// 
#[derive(Debug, PartialEq, Clone)]
pub struct VariableNode {
    pub name: Rc<RefCell<String>>,
}

impl VariableNode {
    pub fn accept(&self, visitor: &mut dyn SyntaxTreeVisitor) {
        visitor.visit_variable(self);
    }

    pub fn rename(&mut self, new_name: &str) {
        self.name = Rc::new(RefCell::new(new_name.to_string()));
    }
}


///
/// Constant Node
/// 
#[derive(Debug, PartialEq, Clone)]
pub struct ConstantNode {
    pub value: i64,
}

impl ConstantNode {
    pub fn accept(&self, visitor: &mut dyn SyntaxTreeVisitor) {
        visitor.visit_constant(self);
    }
}


///
/// Visit Abstraction
/// 
#[derive(Debug, PartialEq, Clone)]
pub struct AbstractionNode {
    pub parameter: Rc<VariableNode>,
    pub body: Rc<ExpressionNode>,
}

impl AbstractionNode {
    pub fn accept(&self, visitor: &mut dyn SyntaxTreeVisitor) {
        visitor.visit_abstraction(self);
    }
}


///
/// Visit Application
/// 
#[derive(Debug, PartialEq, Clone)]
pub struct ApplicationNode {
    pub function: Rc<ExpressionNode>,
    pub argument: Rc<ExpressionNode>,    
}

impl ApplicationNode {
    pub fn accept(&self, visitor: &mut dyn SyntaxTreeVisitor) {
        visitor.visit_application(self);
    }
}


///
/// Arithmetic Node
/// 
#[derive(Debug, PartialEq, Clone)]
pub struct ArithmeticNode {
    pub operator: String,
    pub left: Rc<ExpressionNode>,
    pub right: Rc<ExpressionNode>,
}

impl ArithmeticNode {
    pub fn accept(&self, visitor: &mut dyn SyntaxTreeVisitor) {
        visitor.visit_arithmetic(self);
    }
}


///
/// Let Expression Node
/// 
#[derive(Debug, PartialEq, Clone)]
pub struct LetNode {
    pub expression_lhs: Rc<ExpressionNode>,
    pub expression_rhs: Rc<ExpressionNode>,
    pub body: Rc<ExpressionNode>,
}

impl LetNode {
    pub fn accept(&self, visitor: &mut dyn SyntaxTreeVisitor) {
        visitor.visit_let(self);
    }
}