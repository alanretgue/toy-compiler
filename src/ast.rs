use std::fmt;
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
        Expr::Assign(name, func) => write!(f, "{:?} = {:?}", *name, func),
        Expr::App(name, param) => write!(f, "({:?}{:?})", *name, param),
        Expr::Func(func) => write!(f, "{:?}", *func),
        }
    }
}

impl fmt::Debug for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        Func::ID(id) => write!(f, "{:?}", id),
        Func::Decl(args, e) => write!(f, "(f({:?}) => {:?})", args, *e),
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
            write!(f, " {:?}", param)?;
        }
        Ok(())
    }
}

impl fmt::Debug for ID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Debug for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.args[0])?;
        for i in 1..self.args.len() {
            write!(f, "{:?}", &self.args[i])?;
        }
        Ok(())
    }
}
