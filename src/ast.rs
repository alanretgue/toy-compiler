use std::vec::Vec;

#[derive(PartialEq, Clone)]
pub enum ID {
    Name(String, Box<Expr>),
    Error(Box<ErrorType>, String)
}

#[derive(PartialEq, Clone)]
pub struct Params {
    pub params: Vec<Box<Expr>>,
}

#[derive(PartialEq, Clone)]
pub struct Args {
    pub args: Vec<ID>,
}

#[derive(PartialEq, Clone)]
pub enum Func {
    ID(ID),
    Decl(Args, Box<Expr>),
    Error(Box<ErrorType>),
}

#[derive(PartialEq, Clone)]
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    App(Box<Func>, Params),
    Func(Box<Func>),
    ID(ID),
    Error(Box<ErrorType>),
}

#[derive(PartialEq, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(PartialEq, Clone)]
pub enum ErrorType {
    Success,
    Unhandled,
    VariableNotBinded,
}
