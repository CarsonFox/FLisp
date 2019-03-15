use crate::types::*;
use std::rc::Rc;

use crate::eval::eval;

pub fn add(args: &[Rc<Expression>], env: &mut Environment) -> Result<Rc<Expression>, String> {
    fn add_exprs(acc: Number, x: &Number) -> Number {
        acc + *x
    }
    arithmetic_op(args, env, "add", add_exprs)
}

pub fn sub(args: &[Rc<Expression>], env: &mut Environment) -> Result<Rc<Expression>, String> {
    fn add_exprs(acc: Number, x: &Number) -> Number {
        acc - *x
    }
    arithmetic_op(args, env, "sub", add_exprs)
}

pub fn mul(args: &[Rc<Expression>], env: &mut Environment) -> Result<Rc<Expression>, String> {
    fn add_exprs(acc: Number, x: &Number) -> Number {
        acc * *x
    }
    arithmetic_op(args, env, "sub", add_exprs)
}

pub fn div(args: &[Rc<Expression>], env: &mut Environment) -> Result<Rc<Expression>, String> {
    fn add_exprs(acc: Number, x: &Number) -> Number {
        acc / *x
    }
    arithmetic_op(args, env, "sub", add_exprs)
}

pub fn less_than(args: &[Rc<Expression>], env: &mut Environment) -> Result<Rc<Expression>, String> {
    if args.len() < 2 {
        return Err(String::from("Expected 2 or more arguments to <"));
    }
    Err(String::from("Unimplemented"))
}

fn arithmetic_op(
    args: &[Rc<Expression>],
    env: &mut Environment,
    name: &str,
    op: fn(Number, &Number) -> Number,
) -> Result<Rc<Expression>, String> {
    if args.len() < 2 {
        return Err(format!("Not enough arguments to {}", name));
    }

    //Evaluate all the arguments
    let mut args_eval = Vec::with_capacity(args.len());
    for arg in args {
        args_eval.push(eval(Rc::clone(arg), env)?);
    }

    //Check for non-numeric arguments
    if let Some(expr) = args_eval.iter().find(|expr| !expr.is_number()) {
        return Err(format!("Cannot add non-numeric object {}", expr.as_ref()));
    }

    //Start with first argument, "cast" everything to Number, then sum
    if let Expression::Numeric(first) = args_eval[0].as_ref() {
        let ans = args_eval[1..]
            .iter()
            .map(|expr| {
                if let Expression::Numeric(num) = expr.as_ref() {
                    num
                } else {
                    unreachable!()
                }
            })
            .fold(*first, op);

        return Ok(Rc::new(Expression::Numeric(ans)));
    }

    //Just in case
    let _ = dbg!(args_eval);
    unreachable!()
}
