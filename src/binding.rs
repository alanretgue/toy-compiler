use crate::ast::*;

trait Binding {
    fn bind(&self, hashmap: &mut Vec<HashMap<String, Box<Expr>>>) -> Vec<HashMap<String, Box<Expr>>>;
}

impl Binding for ID {
    fn bind(hashmap: &mut Vec<HashMap<String, Box<Expr>>>) -> Vec<HashMap<String, Box<Expr>>> {
        match self {
            ID::Name(name, e) => {
                if **e == Expr::Error(Box::new(ErrorType::VariableNotBinded)) {
                    if ! hashmap[hashmap.len() - 1].cotains_key(name) {
                        println!("A binding error occured on {}", *name);
                    }
                } else {
                    hashmap[hashmap.len() - 1].insert(*name, **e);
                }
            }
            ID::Error(err, name) => println!("{:?}: {}", *err, name),
        }
        hashmap
    }
}