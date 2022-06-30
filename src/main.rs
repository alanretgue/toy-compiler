#[macro_use] extern crate lalrpop_util;
#[path = "ast.rs"]
mod ast;

lalrpop_mod!(pub calc); // synthesized by LALRPOP

fn main() {
    let expr = calc::StatParser::new().parse("A = 2 * 4").unwrap();
    println!("{:?}", expr);
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

#[test]
fn pretty_print() {
    assert!(format!("{:?}", calc::StatParser::new().parse("(9+1)").unwrap()) == "(9 + 1)");
    assert!(format!("{:?}", calc::StatParser::new().parse("2   *    4").unwrap()) == "(2 * 4)");
    assert!(format!("{:?}", calc::StatParser::new().parse("A = ((4 / 2))").unwrap()) == "A = (4 / 2)");
    assert!(format!("{:?}", calc::StatParser::new().parse("ABC = ((4 / 2) * 23)").unwrap()) == "ABC = ((4 / 2) * 23)");
    assert!(format!("{:?}", calc::StatParser::new().parse("ABC = (4 / (2 * 23))").unwrap()) == "ABC = (4 / (2 * 23))");
    assert!(format!("{:?}", calc::StatParser::new().parse("(A 1)").unwrap()) == "(A 1)");
    assert!(format!("{:?}", calc::StatParser::new().parse("ABC = (A 1)").unwrap()) == "ABC = (A 1)");
}
