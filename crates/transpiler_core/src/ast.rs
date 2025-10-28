
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    ImportStmt(ImportStmt),
    ExportStmt(Box<Statement>),
    VariableDecl(VariableDecl),
    FunctionDecl(FunctionDecl),
    StructDecl(StructDecl),
    EnumDecl(EnumDecl),
    PrintStmt(PrintStmt),
    ReturnStmt(ReturnStmt),
    ExpressionStmt(Expression),
    IfElse(IfElseStmt),
    ForLoop(ForLoopStmt),
    WhileLoop(WhileLoopStmt),
    BreakStmt,
    ContinueStmt,
    TryCatch(TryCatchStmt),
    ThrowStmt(ThrowStmt),
}

#[derive(Debug, Clone)]
pub struct ImportStmt {
    pub imports: Vec<ImportItem>,
    pub path: String,
    pub is_external: bool,
}

#[derive(Debug, Clone)]
pub struct ImportItem {
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VariableDecl {
    pub name: String,
    pub var_type: Option<Type>,
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
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub field_type: Type,
}

#[derive(Debug, Clone)]
pub struct EnumDecl {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Option<Vec<Type>>,
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

#[derive(Debug, Clone)]
pub struct TryCatchStmt {
    pub try_body: Vec<Statement>,
    pub catch_param: Option<String>,
    pub catch_body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct ThrowStmt {
    pub expression: Expression,
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
    Custom(String),
    Inferred,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    NumberLiteral(i32),
    StringLiteral(String),
    BooleanLiteral(bool),
    ArrayLiteral(Vec<Expression>),
    StructLiteral {
        name: String,
        fields: Vec<(String, Expression)>,
    },
    BinaryOp(Box<Expression>, BinaryOp, Box<Expression>),
    FunctionCall(String, Vec<Expression>),
    MethodCall {
        object: Box<Expression>,
        method: String,
        arguments: Vec<Expression>,
    },
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
