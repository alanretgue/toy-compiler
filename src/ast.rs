use std::vec::Vec;

pub struct ID {
    pub name: String,
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
}

pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Assign(ID, Box<Expr>),
    App(Box<Func>, Params),
    Func(Box<Func>),
    ID(ID),
}

pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}
