use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};
use clap::Parser;

#[macro_use] extern crate lalrpop_util;
#[path = "ast.rs"]
mod ast;
#[path = "launchers.rs"]
mod launchers;
#[path = "pretty_print.rs"]
mod pretty_print;
#[path = "binding.rs"]
mod binding;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

// TODO: handle arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   // Flag to execute pretty print
   #[arg(short, long)]
   pretty_print: bool,

   // Flag to execute binding
   #[arg(short, long)]
   binding: bool,

   // Input file
   #[arg(short, long)]
   input: String,
}


fn main() -> Result<(), Error> {
    let cli = Args::parse();

    let result = if cli.pretty_print {
        apply_on_file(&cli.input, launchers::display_pretty_print)
    } else if cli.binding {
        apply_on_file(&cli.input, launchers::display_binding)
    } else {
        apply_on_file(&cli.input, launchers::display_pretty_print)
    };

    if let Err((error_kind, error_message)) = result {
        let err = Error::new(error_kind, error_message);
        return Err(err);
    } else if result.is_ok() {
        return Ok(());
    }
    return Ok(());
}

fn apply_on_file(filename: &str, f: fn(String) -> Result<u8, (ErrorKind, String)>) -> Result<u8, (ErrorKind, String)> {
    let file = File::open(filename);
    if file.is_err() {
        return Err((ErrorKind::InvalidInput, "An error occured on reading the file".to_owned()));
    }
    let file_content = io::BufReader::new(file.unwrap()).lines();
    for line in file_content.flatten() {
        let res = f(line);
        println!("{}", (res?));
    }
    Ok(0)
}

/**
 * UNIT TEST SECTION
 */

fn _launch_parsing(str: &str, error: bool) -> bool {
    let mut errors = Vec::new();
    let _ = parser::StatParser::new().parse(&mut errors, str);

    (error && !errors.is_empty()) || (!error && errors.is_empty())
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

#[test]
fn pretty_print() {
    let test_array = [
        ("(9+1)", "(9 + 1)"),
        ("a + b", "(a + b)"),
        ("2   *    4", "(2 * 4)"),
        ("A = ((4 / 2))", "A = (4 / 2)"),
        ("ABC = ((4 / 2) * 23)", "ABC = ((4 / 2) * 23)"),
        ("ABC = (4 / (2 * 23))", "ABC = (4 / (2 * 23))"),
        ("($A 1)", "($A 1)"),
        ("ABC = ($A 1)", "ABC = ($A 1)"),
        ("ABC = ($A 1 2 3 2 1)", "ABC = ($A 1 2 3 2 1)"),
        ("ABC = 2 + 1 * 4", "ABC = (2 + (1 * 4))"),
        ("ABC = (f(a) => 1 + 1)", "ABC = (f(a) => (1 + 1))"),
        ("ABC = (f(a) => (1 + 1))", "ABC = (f(a) => (1 + 1))"),
        ("ABC = (f(a) => ((f(b, c) => b + c) a 2))", "ABC = (f(a) => ((f(b, c) => (b + c)) a 2))"),
    ];

    for (input, expected) in test_array {
        let res = launchers::launch_pretty_print(input);
        if let Err(_) = res {
            panic!("An error occured on '{}'", input);
        }
        assert_eq!(res.unwrap(), expected);
    }
}
