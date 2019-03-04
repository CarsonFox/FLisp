use crate::types::*;
use std::rc::Rc;

pub fn eval(expr: &Expression, env: &Environment) -> Result<Rc<Atom>, String> {
    match expr {
        Expression::Atomic(atom) => Ok(Rc::clone(atom)),
        Expression::SExpr(list) => apply(list, env),
    }
}

fn apply(list: &Vec<Expression>, env: &Environment) -> Result<Rc<Atom>, String> {
    if list.is_empty() {
        return Err(String::from("Attempted to evaluate empty S-Expression"));
    }

    let first_expr = eval(&list[0], env)?;

    let proc = match first_expr.as_ref() {
        Atom::Identifier(id) => match env.get(id) {
            Some(proc) => proc,
            None => {
                return Err(format!("Attempted to apply undefined procedure: {}", id));
            }
        },
        Atom::Numeric(x) => {
            return Err(format!("Cannot apply Number {} as a procedure", x));
        }
    };

    //Evaluate arguments
    let mut args = Vec::with_capacity(list.len());
    for expr in list[1..].iter() {
        args.push(eval(expr, env)?);
    }

    match proc {
        Procedure::Builtin(builtin) => builtin(args),
    }
}
