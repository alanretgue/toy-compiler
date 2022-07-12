use std::vec::Vec;

pub enum ID {
    Name(String),
    Error(Box<ErrorType>, String)
}

pub struct Params {
    pub params: Vec<Box<Expr>>,
}

pub struct Args {
    pub args: Vec<ID>,
}

pub enum Func {
    ID(ID),
    Decl(Args, Box<Expr>),
    Error(Box<ErrorType>),
}

pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Assign(ID, Box<Expr>),
    App(Box<Func>, Params),
    Func(Box<Func>),
    ID(ID),
    Error(Box<ErrorType>),
}

pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(PartialEq)]
pub enum ErrorType {
    Success,
    InnerVar,
    Outervar,
    Unhandled,
}
