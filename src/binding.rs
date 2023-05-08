use crate::{ast, launchers::launch_binding};
use std::collections::HashMap;

pub trait Binding {
    fn bind(&self, hashmap: &mut Vec<HashMap<String, Box<ast::Expr>>>) -> Vec<HashMap<String, Box<ast::Expr>>>;
}

impl Binding for ast::ID {
    fn bind(&self, hashmap: &mut Vec<HashMap<String, Box<ast::Expr>>>) -> Vec<HashMap<String, Box<ast::Expr>>> {
        match self {
            ast::ID::Name(name, e) => {
                let size = hashmap.len() - 1;
                if **e == ast::Expr::Error(Box::new(ast::ErrorType::VariableNotBinded)) {
                    if ! hashmap[size].contains_key(name) {
                        println!("A binding error occured on {}", *name);
                    }
                } else {
                    hashmap[size].insert(name.to_string(), e.clone());
                }
            },
            ast::ID::Error(err, name) => println!("{:?}: {}", *err, name),
        }
        hashmap.to_vec()
    }
}

impl Binding for ast::Params {
    fn bind(&self, hashmap: &mut Vec<HashMap<String, Box<ast::Expr>>>) -> Vec<HashMap<String, Box<ast::Expr>>> {
        hashmap.to_vec()
    }
}

impl Binding for ast::Args {
    fn bind(&self, hashmap: &mut Vec<HashMap<String, Box<ast::Expr>>>) -> Vec<HashMap<String, Box<ast::Expr>>> {
        for id in self.args.iter() {
            *hashmap = id.bind(hashmap);
        }
        hashmap.to_vec()
    }
}

impl Binding for ast::Func {
    fn bind(&self, hashmap: &mut Vec<HashMap<String, Box<ast::Expr>>>) -> Vec<HashMap<String, Box<ast::Expr>>> {
        match self {
            ast::Func::ID(id) => { *hashmap = id.bind(hashmap); },
            ast::Func::Decl(args, expr) => {
                match hashmap.last() {
                    None => hashmap.push(HashMap::new()),
                    Some(h) => hashmap.push(h.clone())
                }

                *hashmap = args.bind(hashmap);
                *hashmap = expr.bind(hashmap);
            },
            ast::Func::Error(err) => println!("{:?}", *err),
        }
        hashmap.to_vec()
    }
}

impl Binding for ast::Expr {
    fn bind(&self, hashmap: &mut Vec<HashMap<String, Box<ast::Expr>>>) -> Vec<HashMap<String, Box<ast::Expr>>> {
        match self {
            ast::Expr::Number(_) => {},
            ast::Expr::Op(expr1, _, expr2) => {
                *hashmap = expr1.bind(hashmap);
                *hashmap = expr2.bind(hashmap);
            },
            ast::Expr::App(func, params) => {
                *hashmap = func.bind(hashmap);
                *hashmap = params.bind(hashmap);
            },
            ast::Expr::Func(func) => {
                *hashmap = func.bind(hashmap);
            },
            ast::Expr::ID(id) => {
                *hashmap = id.bind(hashmap);
            },
            ast::Expr::Error(err) => println!("{:?}", *err),
        }
        hashmap.to_vec()
    }
}

impl Binding for ast::Opcode {
    fn bind(&self, hashmap: &mut Vec<HashMap<String, Box<ast::Expr>>>) -> Vec<HashMap<String, Box<ast::Expr>>> {
        hashmap.to_vec()
    }
}

#[test]
fn binding_simple() {
    // TODO: Add some tests
    let mut binding_map: Vec<HashMap<String, Box<ast::Expr>>> = Vec::new();

    let line = "1 + 1";

    let result = launch_binding(line, &mut binding_map);

    assert_eq!(result.unwrap().len(), 0);
}
