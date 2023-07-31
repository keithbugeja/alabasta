use std::rc::Rc;

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

#[derive(Debug, PartialEq, Clone)]
pub struct VariableNode {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConstantNode {
    pub value: i64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AbstractionNode {
    pub parameter: Rc<VariableNode>,
    pub body: Rc<ExpressionNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ApplicationNode {
    pub function: Rc<ExpressionNode>,
    pub argument: Rc<ExpressionNode>,    
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArithmeticNode {
    pub operator: String,
    pub left: Rc<ExpressionNode>,
    pub right: Rc<ExpressionNode>,
}


#[derive(Debug, PartialEq, Clone)]
pub struct LetNode {
    pub expression_lhs: Rc<ExpressionNode>,
    pub expression_rhs: Rc<ExpressionNode>,
    pub body: Rc<ExpressionNode>,
}