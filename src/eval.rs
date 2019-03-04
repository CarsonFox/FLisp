use crate::types::*;
use std::rc::Rc;

pub fn eval(expr: Rc<Expression>, env: &Environment) -> Result<Rc<Expression>, String> {
    match expr.as_ref() {
        Expression::Numeric(_) => Ok(Rc::clone(&expr)),
        Expression::Identifier(id) => match env.get(id) {
            Some(expr) => Ok(Rc::clone(expr)),
            None => Err(format!("Unbound variable: {}", id)),
        },
        Expression::SExpr(list) => apply(list, env),
    }
}

fn apply(_list: &Vec<Expression>, _env: &Environment) -> Result<Rc<Expression>, String> {
    Err(String::from("Function application not supported."))
}
