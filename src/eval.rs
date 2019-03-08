use crate::types::*;
use std::collections::HashSet;
use std::rc::Rc;

use lazy_static::lazy_static;

lazy_static! {
    static ref SPECIAL_FORMS: HashSet<&'static str> = ["+", "define"].iter().cloned().collect();
}

pub fn eval(expr: Rc<Expression>, env: &mut Environment) -> Result<Rc<Expression>, String> {
    match expr.as_ref() {
        Expression::Numeric(_) => Ok(Rc::clone(&expr)),
        Expression::Identifier(id) => match env_lookup(id, env) {
            Some(expr) => Ok(Rc::clone(&expr)),
            None => {
                if is_special_form(expr.as_ref()) {
                    Ok(expr)
                } else {
                    Err(format!("Unbound variable: {}", id))
                }
            }
        },
        Expression::SExpr(list) => apply(list, env),
    }
}

fn apply(list: &Vec<Rc<Expression>>, env: &mut Environment) -> Result<Rc<Expression>, String> {
    //General approach: Try to eval first sub-expression. If it can't be evaluated, check to see
    //if it's a special form.
    if list.is_empty() {
        return Err(String::from("Empty application"));
    }

    let result = eval(Rc::clone(&list[0]), env)?;

    match result.as_ref() {
        Expression::Numeric(_) => return Err(String::from("Cannot apply Number as a Procedure.")),
        Expression::Identifier(id) => {
            if let Some(expr) = env_lookup(id, env) {
                return call(Rc::clone(&expr), &list[1..], env);
            } else if let Some(result) = special_form(id, &list[1..], env) {
                return result;
            } else {
                return Err(format!("Unrecognized procedure: {}", id));
            }
        }
        _ => unreachable!(),
    }
}

fn call(
    proc: Rc<Expression>,
    _args: &[Rc<Expression>],
    _env: &mut Environment,
) -> Result<Rc<Expression>, String> {
    return Err(format!("Applying procedure: {}", proc.as_ref()));
}

fn env_lookup(key: &String, env: &Environment) -> Option<Rc<Expression>> {
    for map in env.iter().rev() {
        if let Some(result) = map.get(key.as_str()) {
            return Some(Rc::clone(result));
        }
    }
    None
}

//Check for a special form. Returns None if no special form was found, unless an error occurs.
fn special_form(
    proc: &String,
    args: &[Rc<Expression>],
    env: &mut Environment,
) -> Option<Result<Rc<Expression>, String>> {
    if !SPECIAL_FORMS.contains(proc.as_str()) {
        return None;
    }

    match proc.as_str() {
        "+" => Some(add(args, env)),
        "define" => Some(define(args, env)),
        _ => None,
    }
}

fn is_special_form(expr: &Expression) -> bool {
    match expr {
        Expression::Identifier(id) => SPECIAL_FORMS.contains(id.as_str()),
        _ => false,
    }
}

fn add(args: &[Rc<Expression>], env: &mut Environment) -> Result<Rc<Expression>, String> {
    if args.len() < 2 {
        return Err(String::from("Not enough arguments to addition"));
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
            .fold(*first, |acc, x| acc + *x);

        return Ok(Rc::new(Expression::Numeric(ans)));
    }

    //Just in case
    let _ = dbg!(args_eval);
    unreachable!()
}

fn define(args: &[Rc<Expression>], env: &mut Environment) -> Result<Rc<Expression>, String> {
    if args.len() != 2 {
        return Err(format!("Expected 2 arguments to define, found {}", args.len()));
    }

    match args[0].as_ref() {
        Expression::Identifier(id) => {
            let bind_value = eval(Rc::clone(&args[1]), env)?;
            env.last_mut().unwrap().insert(id.clone(), Rc::clone(&bind_value));
            return Ok(bind_value);
        },
        _ => {
            unimplemented!();
        }
    }
}
