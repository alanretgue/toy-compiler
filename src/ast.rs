use std::fmt;
use std::vec::Vec;

pub struct Params {
    pub params: Vec<Box<Expr>>,
}

pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Assign(Box<String>, Box<Expr>),
    App(Box<String>, Params),
}

pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        Expr::Number(n) => write!(f, "{}", n),
        Expr::Op(first_op, opcode, sec_op) => write!(f, "({:?} {:?} {:?})", first_op, opcode, sec_op),
        Expr::Assign(name, func) => write!(f, "{} = {:?}", *name, func),
        Expr::App(name, param) => write!(f, "({} {:?})", *name, param),
        }
    }
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        Opcode::Mul => write!(f, "*"),
        Opcode::Div => write!(f, "/"),
        Opcode::Add => write!(f, "+"),
        Opcode::Sub => write!(f, "-"),
        }
    }
}

impl fmt::Debug for Params {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for param in &self.params {
            write!(f, "{:?}", param);
        }
        Ok(())
    }
}
