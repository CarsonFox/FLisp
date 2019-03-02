use std::collections::HashMap;
use std::fmt;

//This is almost certainly going to change.
pub type Environment = HashMap<String, Procedure>;

#[derive(Copy, Clone)]
pub enum Procedure {
    Builtin(fn(&[Atom]) -> Result<Atom, String>),
    //The types representing user-defined procs go here
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Atomic(Atom),
    SExpr(Vec<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Integer(i32),
    Float(f32),
    //TODO: Maybe change this to &str? Would need to do lifetime stuff though
    Identifier(String),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Atom::Integer(x) => write!(f, "{}", x),
            Atom::Float(x) => write!(f, "{}", x),
            Atom::Identifier(s) => write!(f, "{}", s),
        }
    }
}
