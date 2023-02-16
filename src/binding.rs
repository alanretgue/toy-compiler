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
            },
            ID::Error(err, name) => println!("{:?}: {}", *err, name),
        }
        hashmap
    }
}

impl Binding for Params {
    fn bind(hashmap: &mut Vec<HashMap<String, Box<Expr>>>) -> Vec<HashMap<String, Box<Expr>>> {
        hashmap
    }
}

impl Binding for Args {
    fn bind(hashmap: &mut Vec<HashMap<String, Box<Expr>>>) -> Vec<HashMap<String, Box<Expr>>> {
        // match hashmap.last() {
        //     None => hashmap.push(HashMap::new()),
        //     Some(h) => hashmap.push(h.clone())
        // }
        for id in self.agrs.iter() {
            hashmap = id.bind(hashmap);
        }
        hashmap
    }
}
