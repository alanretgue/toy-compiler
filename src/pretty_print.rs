use std::fmt;

use crate::ast::*;

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        Expr::Number(n) => write!(f, "{}", n),
        Expr::Op(first_op, opcode, sec_op) => write!(f, "({:?} {:?} {:?})", first_op, opcode, sec_op),
        Expr::App(name, param) => write!(f, "({:?}{:?})", *name, param),
        Expr::Func(func) => write!(f, "{:?}", *func),
        Expr::ID(id) => write!(f, "{:?}", id),
        Expr::Error(err) => write!(f, "{:?}", *err),
        }
    }
}

impl fmt::Debug for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        Func::ID(id) => write!(f, "${:?}", id),
        Func::Decl(args, e) => write!(f, "(f({:?}) => {:?})", args, *e),
        Func::Error(err) => write!(f, "{:?}", *err),
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
        match self {
        ID::Name(name, e) => {
            if **e == Expr::Error(Box::new(ErrorType::VariableNotBinded)) {
                write!(f, "{}", name)
            } else {
                write!(f, "{} = {:?}", name, *e)
            }
        },
        ID::Error(err, name) => write!(f, "{:?}: {}", *err, name),
        }
    }
}

impl fmt::Debug for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.args[0])?;
        for i in 1..self.args.len() {
            write!(f, ", {:?}", &self.args[i])?;
        }
        Ok(())
    }
}

impl fmt::Debug for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        ErrorType::Success => write!(f, "Success"),
        ErrorType::Unhandled => write!(f, "Unhandled Error"),
        ErrorType::VariableNotBinded => write!(f, "Variable not binded"),
        }
    }
}
