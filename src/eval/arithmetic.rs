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
    if args.len() != 2 {
        return Err(format!("Expected 2 arguments to <, found {}", args.len()));
    }

    let arg1 = eval(Rc::clone(&args[0]), env)?;
    let arg2 = eval(Rc::clone(&args[1]), env)?;

    match (arg1.as_ref(), arg2.as_ref()) {
        (Expression::Numeric(n1), Expression::Numeric(n2)) => {
            Ok(Rc::new(Expression::Boolean(n1.less_than(n2))))
        }
        (Expression::Numeric(_), _) => Err(format!(
            "Non-numeric argument to < procedure: {}",
            arg2.as_ref()
        )),
        _ => Err(format!(
            "Non-numeric argument to < procedure: {}",
            arg1.as_ref()
        )),
    }
}

pub fn equal_to(args: &[Rc<Expression>], env: &mut Environment) -> Result<Rc<Expression>, String> {
    if args.len() != 2 {
        return Err(format!("Expected 2 arguments to =, found {}", args.len()));
    }

    let arg1 = eval(Rc::clone(&args[0]), env)?;
    let arg2 = eval(Rc::clone(&args[1]), env)?;

    match (arg1.as_ref(), arg2.as_ref()) {
        (Expression::Numeric(n1), Expression::Numeric(n2)) => {
            Ok(Rc::new(Expression::Boolean(n1.equal_to(n2))))
        }
        (Expression::Numeric(_), _) => Err(format!(
            "Non-numeric argument to = procedure: {}",
            arg2.as_ref()
        )),
        _ => Err(format!(
            "Non-numeric argument to = procedure: {}",
            arg1.as_ref()
        )),
    }
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

pub fn remainder(args: &[Rc<Expression>], env: &mut Environment) -> Result<Rc<Expression>, String> {
    if args.len() != 2 {
        return Err(format!(
            "Expected 2 arguments to remainder, {} were given",
            args.len(),
        ));
    }

    let arg1 = eval(Rc::clone(&args[0]), env)?;
    let arg2 = eval(Rc::clone(&args[1]), env)?;

    if let Expression::Numeric(a) = arg1.as_ref() {
        if let Expression::Numeric(b) = arg2.as_ref() {
            Ok(Rc::new(Expression::Numeric(match a {
                Number::Integer(x) => match b {
                    Number::Integer(y) => Number::Integer(*x % *y),
                    Number::Float(y) => Number::Float(*x as f32 % *y),
                },
                Number::Float(x) => match b {
                    Number::Integer(y) => Number::Float(*x % *y as f32),
                    Number::Float(y) => Number::Float(*x % *y),
                },
            })))
        } else {
            Err(format!(
                "Expected numeric arguments to remainder, got {}",
                arg1.as_ref()
            ))
        }
    } else {
        Err(format!(
            "Expected numeric arguments to remainder, got {}",
            arg2.as_ref()
        ))
    }
}
