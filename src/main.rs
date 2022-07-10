#[macro_use] extern crate lalrpop_util;
#[path = "ast.rs"]
mod ast;

lalrpop_mod!(pub calc); // synthesized by LALRPOP

fn main() {
    let expr = launch_pretty_print("((A) => (1 + 1))");
    println!("{}", expr);
}

#[test]
fn calc() {
    assert!(calc::StatParser::new().parse("A = 12").is_ok());
    assert!(calc::StatParser::new().parse("22").is_ok());
    assert!(calc::StatParser::new().parse("(22)").is_ok());
    assert!(calc::StatParser::new().parse("(2+2)").is_ok());
    assert!(calc::StatParser::new().parse("(2+2) * (2/2)").is_ok());
    assert!(calc::StatParser::new().parse("((((22))))").is_ok());
    assert!(calc::StatParser::new().parse("((22)").is_err());
    assert!(calc::StatParser::new().parse("((2+2)").is_err());
    // assert!(calc::ExprParser::new().parse("A = 2").is_ok());
}

fn launch_pretty_print(str: &str) -> String {
    format!("{:?}", calc::StatParser::new().parse(str).unwrap())
}

#[test]
fn pretty_print() {
    assert_eq!(launch_pretty_print("(9+1)"), "(9 + 1)");
    assert_eq!(launch_pretty_print("2   *    4"), "(2 * 4)");
    assert_eq!(launch_pretty_print("A = ((4 / 2))"), "A = (4 / 2)");
    assert_eq!(launch_pretty_print("ABC = ((4 / 2) * 23)"), "ABC = ((4 / 2) * 23)");
    assert_eq!(launch_pretty_print("ABC = (4 / (2 * 23))"), "ABC = (4 / (2 * 23))");
    assert_eq!(launch_pretty_print("(A 1)"), "(A 1)");
    assert_eq!(launch_pretty_print("ABC = (A 1)"), "ABC = (A 1)");
    assert_eq!(launch_pretty_print("ABC = (A 1 2 3 2 1)"), "ABC = (A 1 2 3 2 1)");
    assert_eq!(launch_pretty_print("ABC = 2 + 1 * 4"), "ABC = (2 + (1 * 4))");
    //assert_eq!(launch_pretty_print("ABC = ((a) => 1 + 1)"), "ABC = ((a) => (1 + 1))");
}
