//Keep things generic, maybe we add multiple precision later
use std::ops::{AddAssign, MulAssign};

extern crate nom;

use nom::{alt, char, do_parse, is_digit, map, multispace0, named, opt, take_while1, ws};

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

fn eval(expr: &Expression) -> i32 {
    match expr {
        Expression::Number(x) => *x,
        Expression::Application(op, args) => apply(op, args),
    }
}

fn apply(op: &Operator, args: &Vec<Expression>) -> i32 {
    //Evaluate the arguments first, then reduce
    //There must be a nicer way to do this, but
    //I don't know how to wrestle with rust's closure
    //bullshit well enough.
    match op {
        Operator::Add => args.iter().map(|expr| eval(expr)).fold(0, |x, y| x + y),
        Operator::Sub => {
            //Subtraction and division must start with the
            //first argument, instead of the identity.
            args.iter()
                .skip(1)
                .map(|expr| eval(expr))
                .fold(eval(&args[0]), |x, y| x - y)
        }
        Operator::Mul => args.iter().map(|expr| eval(expr)).fold(1, |x, y| x * y),
        Operator::Div => args
            .iter()
            .skip(1)
            .map(|expr| eval(expr))
            .fold(eval(&args[0]), |x, y| x / y),
    }
}

fn parse_line(s: &mut String) -> Result<Vec<Expression>, String> {
    //Null terminator to let parsing end. This isn't very neat
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
    Number(i32),
    Application(Operator, Vec<Expression>),
}

named!(expression <&[u8], Expression>, alt!(
    integer => { |x| Expression::Number(x) } |
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

//TODO: check for bad numbers, float support
named!(integer <&[u8], i32>, do_parse!(
    pref: opt!(char!('-')) >>
    num: int_no_prefix >>
    (if pref.is_some() {-num} else {num})
));

named!(int_no_prefix <&[u8], i32>, map!(take_while1!(is_digit), buf_to_int));

//Lifted from nom tutorial, parses a sequence of digits as a number
fn buf_to_int<T>(s: &[u8]) -> T
where
    T: AddAssign + MulAssign + From<u8>,
{
    let mut sum = T::from(0);
    for digit in s {
        sum *= T::from(10);
        sum += T::from(*digit - b'0');
    }
    sum
}

#[test]
fn test_expression() {
    assert_eq!(expression(b"22 "), Ok((&b" "[..], Expression::Number(22))));
    assert_eq!(
        expression(b"-12 "),
        Ok((&b" "[..], Expression::Number(-12)))
    );
    assert_eq!(
        expression(b"( + 2 3 )"),
        Ok((
            &[][..],
            Expression::Application(
                Operator::Add,
                vec![Expression::Number(2), Expression::Number(3)]
            )
        ))
    );
}

#[test]
fn test_operator() {
    assert_eq!(operator(b"+"), Ok((&[][..], Operator::Add)));
    assert_eq!(operator(b"-"), Ok((&[][..], Operator::Sub)));
    assert_eq!(operator(b"*"), Ok((&[][..], Operator::Mul)));
    assert_eq!(operator(b"/"), Ok((&[][..], Operator::Div)));

    assert!(operator(b"a").is_err());
    assert!(operator(b"").is_err());
}

#[test]
fn test_integer() {
    assert_eq!(integer(b"-123456789 "), Ok((&b" "[..], -123456789)));
    assert_eq!(integer(b"123456789)"), Ok((&b")"[..], 123456789)));

    assert!(integer(b"a1").is_err());
    assert!(integer(b"12").is_err());
}
