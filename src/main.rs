#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calc); // synthesized by LALRPOP

fn main() {
    print!("Hello");
}

#[test]
fn calc() {
    assert!(calc::TermParser::new().parse("22").is_ok());
    // assert!(calc::TermParser::new().parse("22)").is_ok());
    assert!(calc::TermParser::new().parse("(22)").is_ok());
    assert!(calc::TermParser::new().parse("((((22))))").is_ok());
    assert!(calc::TermParser::new().parse("((22)").is_err());
}
