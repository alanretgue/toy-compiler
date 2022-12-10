use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};
use clap::Parser;

#[macro_use] extern crate lalrpop_util;
#[path = "ast.rs"]
mod ast;
#[path = "pretty_print.rs"]
mod pretty_print;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long)]
   name: String,

   /// Number of times to greet
   #[arg(short, long, default_value_t = 1)]
   count: u8,
}


fn main() -> Result<(), Error> {
    // let expr = launch_pretty_print("(f(a) => (1 + 1))");
    // println!("{}", expr);

    let result = get_file_content("test/add.lb");

    if let Err(e) = result {
        // println!("{}", e);
        let err = Error::new(ErrorKind::InvalidInput, e);
        return Err(err);
    } else if let Ok(_) = result {
        // println!("{}", r);
        return Ok(());
    }
    // println!("{}", content);
    return Ok(());
}

fn get_file_content(filename: &str) -> Result<u32, &str> {
    let file = File::open(filename);
    if let Err(_) = file {
        return Err("An error occured on reading the file");
    }
    let file_content = io::BufReader::new(file.unwrap()).lines();
    for line in file_content {
        if let Ok(l) = line {
            // println!("{}", l);
            let res = launch_pretty_print(&l);
            println!("{}", res);
        }
    }
    Ok(0)
}

fn _launch_parsing(str: &str, error: bool) -> bool {
    let mut errors = Vec::new();
    let _ = parser::StatParser::new().parse(&mut errors, str);

    (error && errors.len() != 0) || (!error && errors.len() == 0)
}

fn _launch_success(str: &str) -> bool {
    _launch_parsing(str, false)
}

fn _launch_error(str: &str) -> bool {
    _launch_parsing(str, true)
}

#[test]
fn parse_success() {
    assert!(_launch_success("A = 12"));
    assert!(_launch_success("A = B"));
    assert!(_launch_success("22"));
    assert!(_launch_success("(22)"));
    assert!(_launch_success("(2+2)"));
    assert!(_launch_success("(2+2) * (2/2)"));
    assert!(_launch_success("((((22))))"));
    assert!(_launch_success("A = (f(a) => a)"));
}

#[test]
fn parse_error() {
    assert!(_launch_error("((22)"));
    assert!(_launch_error("((2+2)"));
    assert!(_launch_error("A = B = (2+2)"));
    assert!(_launch_error("A = (A 1) = (2+2)"));
    assert!(_launch_error("A = (f(A) => A))"));
}

fn launch_pretty_print(str: &str) -> String {
    let mut errors = Vec::new();
    format!("{:?}", parser::StatParser::new().parse(&mut errors, str).unwrap())
}

#[test]
fn pretty_print() {
    assert_eq!(launch_pretty_print("(9+1)"), "(9 + 1)");
    assert_eq!(launch_pretty_print("a + b"), "(a + b)");
    assert_eq!(launch_pretty_print("2   *    4"), "(2 * 4)");
    assert_eq!(launch_pretty_print("A = ((4 / 2))"), "A = (4 / 2)");
    assert_eq!(launch_pretty_print("ABC = ((4 / 2) * 23)"), "ABC = ((4 / 2) * 23)");
    assert_eq!(launch_pretty_print("ABC = (4 / (2 * 23))"), "ABC = (4 / (2 * 23))");
    assert_eq!(launch_pretty_print("($A 1)"), "($A 1)");
    assert_eq!(launch_pretty_print("ABC = ($A 1)"), "ABC = ($A 1)");
    assert_eq!(launch_pretty_print("ABC = ($A 1 2 3 2 1)"), "ABC = ($A 1 2 3 2 1)");
    assert_eq!(launch_pretty_print("ABC = 2 + 1 * 4"), "ABC = (2 + (1 * 4))");
    assert_eq!(launch_pretty_print("ABC = (f(a) => 1 + 1)"), "ABC = (f(a) => (1 + 1))");
    assert_eq!(launch_pretty_print("ABC = (f(a) => (1 + 1))"), "ABC = (f(a) => (1 + 1))");
    assert_eq!(launch_pretty_print("ABC = (f(a) => ((f(b, c) => b + c) a 2))"), "ABC = (f(a) => ((f(b, c) => (b + c)) a 2))");
}
