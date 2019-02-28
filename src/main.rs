use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

extern crate nom;

#[allow(unused_imports)]
use nom::{
    alt, char, delimited, do_parse, flat_map, multispace0, named, parse_to, take_till1,
    take_while1, ws,
};

extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut ed = Editor::<()>::new();
    loop {
        match ed.readline(">> ") {
            Ok(mut line) => {
                ed.add_history_entry(line.as_ref());

                line.push(char::from(0));
                let _ = dbg!(expression(&line));
            }
            Err(ReadlineError::Interrupted) => {
                println!("Encountered ^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Encountered EOF");
                break;
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}

//named!(skip_whitespace <&[u8], ()>, do_parse!(
//    multispace0 >>
//    (())
//));

named!(expression <&str, Expression>, alt!(
    atom => { |a| Expression::Atomic(a) } |
    sexpr => { |e| Expression::SExpr(e) }
));

named!(sexpr <&str, Vec<Expression> >, delimited!(
    char!('('),
    ws!(many1!(expression)),
    char!(')')
));

#[derive(Debug, PartialEq, Clone)]
enum Expression {
    Atomic(Atom),
    SExpr(Vec<Expression>),
}

named!(atom <&str, Atom>, alt!(
    integer => { |x| Atom::Numeric(Number::Integer(x)) } |
    float   => { |x| Atom::Numeric(Number::Float(x)) } |
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

#[derive(Debug, PartialEq, Clone)]
enum Atom {
    Numeric(Number),
    Identifier(String),
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Number {
    Integer(i32),
    Float(f32),
}

impl Add for Number {
    type Output = Number;

    fn add(self, other: Number) -> Number {
        match self {
            Number::Float(x) => match other {
                Number::Float(y) => Number::Float(x + y),
                Number::Integer(y) => Number::Float(x + y as f32),
            },
            Number::Integer(x) => match other {
                Number::Float(y) => Number::Float(x as f32 + y),
                Number::Integer(y) => Number::Integer(x + y),
            },
        }
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self, other: Number) -> Number {
        match self {
            Number::Float(x) => match other {
                Number::Float(y) => Number::Float(x - y),
                Number::Integer(y) => Number::Float(x - y as f32),
            },
            Number::Integer(x) => match other {
                Number::Float(y) => Number::Float(x as f32 - y),
                Number::Integer(y) => Number::Integer(x - y),
            },
        }
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, other: Number) -> Number {
        match self {
            Number::Float(x) => match other {
                Number::Float(y) => Number::Float(x * y),
                Number::Integer(y) => Number::Float(x * y as f32),
            },
            Number::Integer(x) => match other {
                Number::Float(y) => Number::Float(x as f32 * y),
                Number::Integer(y) => Number::Integer(x * y),
            },
        }
    }
}

impl Div for Number {
    type Output = Number;

    fn div(self, other: Number) -> Number {
        match self {
            Number::Float(x) => match other {
                Number::Float(y) => Number::Float(x / y),
                Number::Integer(y) => Number::Float(x / y as f32),
            },
            Number::Integer(x) => match other {
                Number::Float(y) => Number::Float(x as f32 / y),
                Number::Integer(y) => Number::Integer(x / y),
            },
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Integer(x) => write!(f, "{}", x),
            Number::Float(x) => write!(f, "{}", x),
        }
    }
}

//Detects seperator characters, including null-terminator.
//Should play well with nom's manyX! family of macros.
fn is_seperator(c: char) -> bool {
    c.is_whitespace() || c == ')' || c == '(' || c == char::from(0)
}
