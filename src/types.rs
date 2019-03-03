use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::ops::Add;

//This is almost certainly going to change.
pub type Environment = HashMap<String, Procedure>;

#[derive(Copy, Clone)]
pub enum Procedure {
    Builtin(fn(&[Atom]) -> Result<Atom, String>),
    //The types representing user-defined procs go here
}

#[derive(Debug, Clone)]
pub enum Expression {
    Atomic(Atom),
    SExpr(VecDeque<Expression>),
}

#[derive(Debug, Clone)]
pub enum Atom {
    Numeric(Number),
    Identifier(String),
}

impl Atom {
    pub fn is_number(&self) -> bool {
        match self {
            Atom::Numeric(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Atom::Numeric(x) => write!(f, "{}", x),
            Atom::Identifier(s) => write!(f, "{}", s),
        }
    }
}

//Even though it seems nicer to have all the atoms under one enum at first,
//This causes massive match expressions when trying to implement arithmetic.
//This also lets Number implement Copy, which should be convenient.
#[derive(Debug, Copy, Clone)]
pub enum Number {
    Integer(i32),
    Float(f32),
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Integer(x) => write!(f, "{}", x),
            Number::Float(x) => write!(f, "{}", x),
        }
    }
}

impl Add for Number {
    type Output = Number;
    fn add(self, other: Number) -> Number {
        match self {
            Number::Integer(x) => match other {
                Number::Integer(y) => Number::Integer(x + y),
                Number::Float(y) => Number::Float(x as f32 + y),
            },
            Number::Float(x) => match other {
                Number::Integer(y) => Number::Float(x + y as f32),
                Number::Float(y) => Number::Float(x + y),
            },
        }
    }
}
