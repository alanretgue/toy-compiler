use std::io::ErrorKind;

use crate::{ast, parser};

pub fn launch_pretty_print(str: &str) -> Result<String, (ErrorKind, String)>{
    let parsed = launch_parser(&str)?;
    Ok(format!("{:?}", parsed))
}

pub fn launch_parser(str: &str) -> Result<Box<ast::Expr>, (ErrorKind, String)> {
    let mut errors = Vec::new();
    let parsed = parser::StatParser::new().parse(&mut errors, str);

    if errors.len() != 0 {
        return Err((ErrorKind::InvalidData, "An error occured during parsing".to_owned()));
    }
    return Ok(parsed.unwrap());
}

pub fn display_pretty_print(str: String) -> Result<u8, (ErrorKind, String)> {
    println!("{:?}", launch_pretty_print(&str)?);
    Ok(0)
}