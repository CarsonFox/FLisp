use std::collections::{HashMap, VecDeque};
use std::fmt;

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
