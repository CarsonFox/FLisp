use std::collections::HashMap;
use std::fmt;
use std::ops::Add;
use std::rc::Rc;

pub type Environment = HashMap<String, Rc<Expression>>;

#[derive(Debug, Clone)]
pub enum Expression {
    Numeric(Number),
    Identifier(String),
    SExpr(Vec<Rc<Expression>>),
}

impl Expression {
    pub fn is_number(&self) -> bool {
        match self {
            Expression::Numeric(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Numeric(x) => write!(f, "{}", x),
            Expression::Identifier(s) => write!(f, "{}", s),
            Expression::SExpr(_) => write!(f, "S-Expression"),
        }
    }
}

impl From<i32> for Expression {
    fn from(x: i32) -> Expression {
        Expression::Numeric(Number::Integer(x))
    }
}

impl From<f32> for Expression {
    fn from(x: f32) -> Expression {
        Expression::Numeric(Number::Float(x))
    }
}

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
