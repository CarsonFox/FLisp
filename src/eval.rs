use crate::types::*;
use std::rc::Rc;

pub fn eval(expr: Rc<Expression>, env: &Environment) -> Result<Rc<Atom>, String> {
    match expr.as_ref() {
        Expression::Atomic(atom) => Ok(Rc::clone(atom)),
        Expression::SExpr(list) => apply(Rc::clone(list), env),
    }
}

fn apply(list: Rc<Vec<Expression>>, env: &Environment) -> Result<Rc<Atom>, String> {
    if list.is_empty() {
        return Err(String::from("Attempted to evaluate empty S-Expression"));
    }

//    let atom = eval(list.pop_front().unwrap(), env)?;
//
//    let proc = match atom {
//        Atom::Identifier(id) => match env.get(&id) {
//            Some(proc) => proc,
//            None => {
//                return Err(format!("Attempted to apply undefined procedure: {}", id));
//            }
//        },
//        Atom::Numeric(x) => {
//            return Err(format!("Cannot apply Number {} as a procedure", x));
//        }
//    };
//
//    //Evaluate arguments
//    let mut args = Vec::with_capacity(list.len());
//    for expr in list.into_iter() {
//        args.push(eval(expr, env)?);
//    }
//
//    match proc {
//        Procedure::Builtin(builtin) => builtin(&args[..]),
//    }

    Ok(Rc::new(Atom::Numeric(Number::Integer(0))))
}
