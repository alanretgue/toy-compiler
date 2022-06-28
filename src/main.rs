#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calc); // synthesized by LALRPOP

fn main() {
    let expr = calc::ExprParser::new().parse("A = 3 * 2 + 2").unwrap();

    println!("{}", expr);
}

#[test]
fn calc() {
    assert!(calc::ExprParser::new().parse("A = 12").is_ok());
    assert!(calc::ExprParser::new().parse("22").is_ok());
    assert!(calc::ExprParser::new().parse("(22)").is_ok());
    assert!(calc::ExprParser::new().parse("(2+2)").is_ok());
    assert!(calc::ExprParser::new().parse("(2+2) * (2/2)").is_ok());
    assert!(calc::ExprParser::new().parse("((((22))))").is_ok());
    assert!(calc::ExprParser::new().parse("((22)").is_err());
    assert!(calc::ExprParser::new().parse("((2+2)").is_err());
    // assert!(calc::ExprParser::new().parse("A = 2").is_ok());
}
