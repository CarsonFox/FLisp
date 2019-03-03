use crate::types::*;
use std::collections::VecDeque;

pub fn eval(expr: Expression, env: &Environment) -> Result<Atom, String> {
    match expr {
        Expression::Atomic(atom) => Ok(atom),
        Expression::SExpr(list) => apply(list, env),
    }
}

fn apply(list: VecDeque<Expression>, env: &Environment) -> Result<Atom, String> {
    Ok(Atom::Identifier(String::from("apply")))
}
