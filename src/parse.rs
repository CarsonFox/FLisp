use crate::types::*;
use std::rc::Rc;

#[allow(unused_imports)]
use nom::{
    alt, char, delimited, do_parse, flat_map, multispace0, named, parse_to, tag, take_till1,
    take_while1, ws,
};

//Top level expressions don't need to be Rc.
pub fn parse_repl_line(mut line: String) -> Result<Vec<Expression>, String> {
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
                expr_vec.push(expr);
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

named!(expression <&str, Expression>, alt!(
    atom => { |a| Expression::Atomic(Rc::new(a)) } |
    sexpr => { |e| Expression::SExpr(Rc::new(e)) }
));

named!(sexpr <&str, Vec<Expression> >, delimited!(
    char!('('),
    ws!(many1!(expression)),
    char!(')')
));

named!(atom <&str, Atom>, alt!(
    integer => { |x| Atom::from(x) } |
    float   => { |x| Atom::from(x) } |
    token   => { |x: &str| Atom::Identifier(String::from(x)) }
));

named!(integer <&str, i32>, flat_map!(
    token,
    parse_to!(i32)));

named!(float <&str, f32>, flat_map!(
    token,
    parse_to!(f32)));

named!(token <&str, &str>, take_till1!(
    is_seperator));

//Detects seperator characters, including null-terminator.
fn is_seperator(c: char) -> bool {
    c.is_whitespace() || c == ')' || c == '(' || c == char::from(0)
}
