use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[allow(unused_imports)]
use nom::{
    alt, char, delimited, do_parse, flat_map, multispace0, named, parse_to, tag, take_till1,
    take_while1, ws,
};

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

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Numeric(Number),
    //TODO: Maybe change this to &str? Would need to do lifetime stuff though
    Identifier(String),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Atom::Numeric(x) => write!(f, "{}", x),
            Atom::Identifier(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Number {
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

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Atomic(Atom),
    SExpr(Vec<Expression>),
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

//Detects seperator characters, including null-terminator.
//Should play well with nom's manyX! family of macros.
fn is_seperator(c: char) -> bool {
    c.is_whitespace() || c == ')' || c == '(' || c == char::from(0)
}
