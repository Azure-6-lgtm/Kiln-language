#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Import(Vec<String>),
    Function(Function),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: String,
        mutable: bool,
        value: Expr,
    },
    Return(Option<Expr>),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(String),
    String(String),
    Variable(String),
    Boolean(bool),

    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },

    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,

    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}
