
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
    IfElse(IfElseStmt),
    ForLoop(ForLoopStmt),
    WhileLoop(WhileLoopStmt),
    BreakStmt,
    ContinueStmt,
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

#[derive(Debug, Clone)]
pub struct IfElseStmt {
    pub condition: Expression,
    pub then_body: Vec<Statement>,
    pub else_body: Option<Vec<Statement>>,
}

#[derive(Debug, Clone)]
pub struct ForLoopStmt {
    pub variable: String,
    pub iterable: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct WhileLoopStmt {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    String,
    Boolean,
    Void,
    Any,
    Array {
        element_type: Box<Type>,
        size: Option<usize>,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    NumberLiteral(i32),
    StringLiteral(String),
    BooleanLiteral(bool),
    ArrayLiteral(Vec<Expression>),
    BinaryOp(Box<Expression>, BinaryOp, Box<Expression>),
    FunctionCall(String, Vec<Expression>),
    IndexAccess {
        object: Box<Expression>,
        index: Box<Expression>,
    },
    MemberAccess {
        object: Box<Expression>,
        member: String,
    },
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
