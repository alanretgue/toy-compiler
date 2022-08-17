use std::vec::Vec;

#[derive(PartialEq)]
pub enum ID {
    Name(String, Box<Expr>),
    Error(Box<ErrorType>, String)
}

#[derive(PartialEq)]
pub struct Params {
    pub params: Vec<Box<Expr>>,
}

#[derive(PartialEq)]
pub struct Args {
    pub args: Vec<ID>,
}

#[derive(PartialEq)]
pub enum Func {
    ID(ID),
    Decl(Args, Box<Expr>),
    Error(Box<ErrorType>),
}

#[derive(PartialEq)]
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    App(Box<Func>, Params),
    Func(Box<Func>),
    ID(ID),
    Error(Box<ErrorType>),
}

#[derive(PartialEq)]
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
    VariableNotBinded,
}
