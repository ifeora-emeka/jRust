
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDecl(VariableDecl),
    FunctionDecl(FunctionDecl),
    PrintStmt(PrintStmt),
    ReturnStmt(ReturnStmt),
    ExpressionStmt(Expression),
}

#[derive(Debug, Clone)]
pub struct VariableDecl {
    pub name: String,
    pub var_type: Type,
    pub value: Expression,
    pub is_const: bool,
}

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
}

#[derive(Debug, Clone)]
pub struct PrintStmt {
    pub expression: Expression,
}

#[derive(Debug, Clone)]
pub struct ReturnStmt {
    pub value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    String,
    Boolean,
    Void,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    NumberLiteral(i32),
    StringLiteral(String),
    BinaryOp(Box<Expression>, BinaryOp, Box<Expression>),
    FunctionCall(String, Vec<Expression>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
}
