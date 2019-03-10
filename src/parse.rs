use std::collections::HashMap;
use std::rc::Rc;

use nom::{alt, char, delimited, flat_map, named, parse_to, tag, take_till1, ws};

use crate::eval::*;
use crate::types::*;

pub fn parse_repl_line(mut line: String) -> Result<Vec<Rc<Expression>>, String> {
    //Needs to be null-terminated to play well with nom
    line.push(char::from(0));

    let mut slice = line.trim_start();
    let mut expr_vec = Vec::new();

    loop {
        //The end of the line has been reached
        if slice.starts_with(char::from(0)) {
            return Ok(expr_vec);
        }

        match expression(slice) {
            Ok((remainder, expr)) => {
                expr_vec.push(Rc::clone(&expr));
                slice = remainder.trim_start();
            }
            Err(nom::Err::Incomplete(_)) => {
                return Err(format!(
                    "Incomplete input: {}",
                    slice.trim_end_matches(char::from(0))
                ));
            }
            Err(_) => {
                return Err(format!(
                    "Error parsing: {}",
                    slice.trim_end_matches(char::from(0))
                ));
            }
        }
    }
}

pub fn load_stdlib(bytes: &[u8]) -> Environment {
    let mut env = vec![HashMap::new()];
    let mut buf = String::from_utf8(bytes.to_vec()).unwrap();
    buf.push(char::from(0));

    let mut slice = buf.trim_start();

    loop {
        //The end of the line has been reached
        if slice.starts_with(char::from(0)) {
            return env;
        }

        match expression(slice) {
            Ok((remainder, expr)) => {
                //Ignore result of evaluation
                let _ = eval(Rc::clone(&expr), &mut env);
                slice = remainder.trim_start();
            }
            Err(nom::Err::Incomplete(_)) => {
                panic!(format!(
                    "Incomplete stdandard library: {}",
                    slice.trim_end_matches(char::from(0))
                ));
            }
            Err(_) => {
                panic!(format!(
                    "Error parsing standard library: {}",
                    slice.trim_end_matches(char::from(0))
                ));
            }
        }
    }
}

named!(expression <&str, Rc<Expression>>, alt!(
    atom => { |a| a } |
    sexpr => { |e| Rc::new(Expression::SExpr(e)) }
));

named!(sexpr <&str, Vec<Rc<Expression>>>, delimited!(
    char!('('),
    ws!(many1!(expression)),
    char!(')')
));

named!(atom <&str, Rc<Expression>>, alt!(
    integer => { |i| Rc::new(Expression::from(i)) } |
    float   => { |f| Rc::new(Expression::from(f)) } |
    boolean => { |b| Rc::new(Expression::Boolean(b)) } |
    token   => { |tok: &str| Rc::new(Expression::Identifier(String::from(tok))) }
));

named!(integer <&str, i32>, flat_map!(
    token,
    parse_to!(i32)));

named!(float <&str, f32>, flat_map!(
    token,
    parse_to!(f32)));

named!(boolean <&str, bool>, alt!(
    tag!("#t") => { |_| true } |
    tag!("#f") => { |_| false }
));

named!(token <&str, &str>, take_till1!(
    is_seperator));

//Detects seperator characters, including null-terminator.
fn is_seperator(c: char) -> bool {
    c.is_whitespace() || c == ')' || c == '(' || c == char::from(0)
}
