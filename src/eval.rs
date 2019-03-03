use crate::types::*;
use std::collections::VecDeque;

//All this code should probably be written to work on Rc pointers or something.
//Right now every time a defined procedure is evaluated it will have to be
//deep cloned from the environment.

pub fn eval(expr: Expression, env: &Environment) -> Result<Atom, String> {
    match expr {
        Expression::Atomic(atom) => Ok(atom),
        Expression::SExpr(list) => apply(list, env),
    }
}

fn apply(mut list: VecDeque<Expression>, env: &Environment) -> Result<Atom, String> {
    if list.is_empty() {
        return Err(String::from("Attempted to evaluate empty S-Expression"));
    }

    let atom = eval(list.pop_front().unwrap(), env)?;

    let proc = match atom {
        Atom::Identifier(id) => match env.get(&id) {
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
    for expr in list.into_iter() {
        args.push(eval(expr, env)?);
    }

    match proc {
        Procedure::Builtin(builtin) => builtin(&args[..]),
    }
}
