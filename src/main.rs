use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

extern crate nom;

#[allow(unused_imports)]
use nom::{
    alt, char, delimited, do_parse, flat_map, multispace0, named, parse_to, tag, take_till1,
    take_while1, ws,
};

extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

const NULL: char = char::from(0);

fn main() {
    let mut ed = Editor::<()>::new();
    loop {
        match ed.readline(">> ") {
            Ok(line) => {
                ed.add_history_entry(line.as_ref());
                let _ = dbg!(parse_repl_line(line));
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

fn parse_repl_line(mut line: String) -> Result<Vec<Expression>, String> {
    //Needs to be null-terminated to play well with nom
    line.push(NULL);

    let mut slice = line.trim_start();
    let mut expr_vec = Vec::new();

    loop {
        //The end of the line has been reached
        if slice.starts_with(NULL) {
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
                    slice.trim_end_matches(NULL)
                ));
            }
            Err(_) => {
                return Err(format!(
                    "Error parsing: {}",
                    slice.trim_end_matches(NULL)
                ));
            }
        }
    }
}

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
    //TODO: Maybe change this to &str? Would need to do lifetime stuff though
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
    c.is_whitespace() || c == ')' || c == '(' || c == NULL
}
