use crate::ast::*;
use std::collections::HashMap;

trait Binding {
    fn bind(&self, hashmap: &mut Vec<HashMap<String, Box<Expr>>>) -> Vec<HashMap<String, Box<Expr>>>;
}

impl Binding for ID {
    fn bind(&self, hashmap: &mut Vec<HashMap<String, Box<Expr>>>) -> Vec<HashMap<String, Box<Expr>>> {
        match self {
            ID::Name(name, e) => {
                let size = hashmap.len() - 1;
                if **e == Expr::Error(Box::new(ErrorType::VariableNotBinded)) {
                    if ! hashmap[size].contains_key(name) {
                        println!("A binding error occured on {}", *name);
                    }
                } else {
                    hashmap[size].insert(name.to_string(), e.clone());
                }
            },
            ID::Error(err, name) => println!("{:?}: {}", *err, name),
        }
        hashmap.to_vec()
    }
}

impl Binding for Params {
    fn bind(&self, hashmap: &mut Vec<HashMap<String, Box<Expr>>>) -> Vec<HashMap<String, Box<Expr>>> {
        hashmap.to_vec()
    }
}

impl Binding for Args {
    fn bind(&self, hashmap: &mut Vec<HashMap<String, Box<Expr>>>) -> Vec<HashMap<String, Box<Expr>>> {
        for id in self.args.iter() {
            *hashmap = id.bind(hashmap);
        }
        hashmap.to_vec()
    }
}

impl Binding for Func {
    fn bind(&self, hashmap: &mut Vec<HashMap<String, Box<Expr>>>) -> Vec<HashMap<String, Box<Expr>>> {
        match self {
            Func::ID(id) => { *hashmap = id.bind(hashmap); },
            Func::Decl(args, expr) => {
                match hashmap.last() {
                    None => hashmap.push(HashMap::new()),
                    Some(h) => hashmap.push(h.clone())
                }

                *hashmap = args.bind(hashmap);
                *hashmap = expr.bind(hashmap);
            },
            Func::Error(err) => println!("{:?}", *err),
        }
        hashmap.to_vec()
    }
}

impl Binding for Expr {
    fn bind(&self, hashmap: &mut Vec<HashMap<String, Box<Expr>>>) -> Vec<HashMap<String, Box<Expr>>> {
        match self {
            Expr::Number(_) => {},
            Expr::Op(expr1, _, expr2) => {
                *hashmap = expr1.bind(hashmap);
                *hashmap = expr2.bind(hashmap);
            },
            Expr::App(func, params) => {
                *hashmap = func.bind(hashmap);
                *hashmap = params.bind(hashmap);
            },
            Expr::Func(func) => {
                *hashmap = func.bind(hashmap);
            },
            Expr::ID(id) => {
                *hashmap = id.bind(hashmap);
            },
            Expr::Error(err) => println!("{:?}", *err),
        }
        hashmap.to_vec()
    }
}
