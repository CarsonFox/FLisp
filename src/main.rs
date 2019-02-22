use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

extern crate nom;

use nom::{
    alt, char, do_parse, flat_map, is_digit, multispace0, named, opt, parse_to, preceded,
    recognize, take_while1, ws,
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

                match parse_line(&mut line) {
                    Ok(vec) => {
                        for expr in vec.iter() {
                            println!("{}", eval(expr));
                        }
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                    }
                }
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

fn eval(expr: &Expression) -> Number {
    match expr {
        Expression::Application(op, args) => apply(op, args),
        Expression::Numeric(num) => *num,
    }
}

fn apply(op: &Operator, args: &Vec<Expression>) -> Number {
    //Evaluate all but the first part of the paren expression
    let iter = args.iter().skip(1).map(|expr| eval(expr));

    //Evaluate the first expression, use it to perform the correct operation
    match op {
        Operator::Add => iter.fold(eval(&args[0]), |x, y| x + y),
        Operator::Sub => iter.fold(eval(&args[0]), |x, y| x - y),
        Operator::Mul => iter.fold(eval(&args[0]), |x, y| x * y),
        Operator::Div => iter.fold(eval(&args[0]), |x, y| x / y),
    }
}

fn parse_line(s: &mut String) -> Result<Vec<Expression>, String> {
    //Null terminator to let parsing end. This doesn't seem like the correct method...
    s.push(char::from(0));

    match line(s.as_bytes()) {
        Ok((_, vec)) => Ok(vec),

        Err(err) => Err(match err {
            nom::Err::Incomplete(_) => String::from("Incomplete"),
            nom::Err::Error(_) => String::from("Error!"),
            nom::Err::Failure(_) => String::from("Failure!"),
        }),
    }
}

named!(line <&[u8], Vec<Expression> >, ws!(many0!(expression)));

#[derive(Debug, PartialEq, Clone)]
enum Expression {
    Numeric(Number),
    Application(Operator, Vec<Expression>),
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

named!(expression <&[u8], Expression>, alt!(
    float => { |x| Expression::Numeric(Number::Float(x)) } |
    integer => { |x| Expression::Numeric(Number::Integer(x)) } |
    do_parse!(
        char!('(') >>
        multispace0 >>
        op: operator >>
        expr_list: ws!(many0!(expression)) >>
        char!(')') >>
        (Expression::Application(op, expr_list))
    )
));

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

named!(operator <&[u8], Operator>, alt!(
    char!('+') => { |_| Operator::Add } |
    char!('-') => { |_| Operator::Sub } |
    char!('*') => { |_| Operator::Mul } |
    char!('/') => { |_| Operator::Div }
));

named!(float <&[u8], f32>, flat_map!(
    recognize!(
        do_parse!(
            opt!(char!('-')) >>
            take_while1!(is_digit) >>
            char!('.') >>
            take_while1!(is_digit) >>
            ()
        )),
    parse_to!(f32)));

named!(integer <&[u8], i32>, flat_map!(
    recognize!(
        preceded!(
            opt!(char!('-')),
            take_while1!(is_digit))),
    parse_to!(i32)));
