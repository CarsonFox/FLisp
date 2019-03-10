use crate::types::*;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use lazy_static::lazy_static;

lazy_static! {
    static ref SPECIAL_FORMS: HashSet<&'static str> =
        ["+", "define", "cond", "if"].iter().cloned().collect();
}

pub fn eval(expr: Rc<Expression>, env: &mut Environment) -> Result<Rc<Expression>, String> {
    match expr.as_ref() {
        Expression::Numeric(_) | Expression::Boolean(_) => Ok(Rc::clone(&expr)),
        Expression::Identifier(id) => match env_lookup(id, env) {
            Some(expr) => Ok(Rc::clone(&expr)),
            None => Err(format!("Unbound variable: {}", id)),
        },
        Expression::SExpr(list) => apply(list, env),
        _ => {
            panic!("Should never evaluate a procedure!");
        }
    }
}

fn apply(list: &Vec<Rc<Expression>>, env: &mut Environment) -> Result<Rc<Expression>, String> {
    if list.is_empty() {
        return Err(String::from("Empty application"));
    }

    //Try to evaluate the first sub-expression
    let result = eval(Rc::clone(&list[0]), env);

    //If evaluation fails, check for a special form
    if result.is_err() {
        if let Expression::Identifier(id) = list[0].as_ref() {
            if let Some(spec_result) = special_form(id, &list[1..], env) {
                return spec_result;
            }
        } else {
            //If the procedure isn't a special form, pass on the error from eval
            return result;
        }
    }

    assert!(result.is_ok());

    //Evaluation succeeded, try to call procedure
    //Get arguments from list of identifiers
    let args = &list[1..];

    match result.unwrap().as_ref() {
        Expression::Procedure(proc) => {
            //Check that arity matches provided args
            if proc.arity() == args.len() {
                //Create new stack frame, fill in args
                let mut frame = HashMap::with_capacity(args.len());
                for (key, value) in proc.get_arg_ids().iter().zip(args.iter()) {
                    frame.insert(key.clone(), Rc::clone(value));
                }

                //Push the frame, evaluate procedure
                env.push(frame);
                let result = eval(proc.get_body(), env);

                //Pop the stack frame, return result
                env.pop();
                result
            } else {
                Err(format!(
                    "Expected {} arguments, but {} were provided.",
                    proc.arity(),
                    args.len()
                ))
            }
        }
        Expression::Numeric(num) => Err(format!("Cannot apply Number {} as a Procedure.", num)),
        Expression::Boolean(b) => Err(format!("Cannot apply boolean {} as a Procedure.", b)),
        //Eval should never return an Identifier or S-Expression!
        Expression::Identifier(_) => unreachable!(),
        Expression::SExpr(_) => unreachable!(),
    }
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
        "cond" => Some(cond(args, env)),
        "if" => Some(s_if(args, env)),
        _ => None,
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
        return Err(format!(
            "Expected 2 arguments to define, found {}",
            args.len()
        ));
    }

    match args[0].as_ref() {
        Expression::Identifier(id) => {
            let bind_value = eval(Rc::clone(&args[1]), env)?;

            env.last_mut()
                .unwrap()
                .insert(id.clone(), Rc::clone(&bind_value));

            return Ok(bind_value);
        }
        Expression::SExpr(sexpr) => {
            //Check for non-identifier arguments
            if let Some(expr) = sexpr.iter().find(|expr| !expr.is_identifier()) {
                return Err(format!(
                    "Expected list of identifiers, found {}",
                    expr.as_ref()
                ));
            }

            let proc = Rc::new(Expression::Procedure(Procedure::new(
                sexpr[1..]
                    .iter()
                    .map(|expr| {
                        if let Expression::Identifier(id) = expr.as_ref() {
                            id.clone()
                        } else {
                            unreachable!()
                        }
                    })
                    .collect(),
                Rc::clone(&args[1]),
            )));

            if let Expression::Identifier(id) = sexpr[0].as_ref() {
                env.last_mut().unwrap().insert(id.clone(), Rc::clone(&proc));
                return Ok(proc);
            }
            unreachable!()
        }
        _ => {
            unimplemented!();
        }
    }
}

// cond looks at a list of pairs - predicates and values. It evaluates each predicate until
// one returns #t, then returns the corresponding value.
fn cond(args: &[Rc<Expression>], env: &mut Environment) -> Result<Rc<Expression>, String> {
    if args.is_empty() {
        return Err(String::from("Empty conditional"));
    }

    for expr in args.iter() {
        match expr.as_ref() {
            Expression::SExpr(pair) => {
                if pair.len() != 2 {
                    return Err(format!("Expected pair in cond expression, found {} expressions", pair.len()));
                }

                match eval(Rc::clone(&pair[0]), env) {
                    Ok(expr) => match expr.as_ref() {
                        Expression::Boolean(b) => {
                            if *b {
                                return eval(Rc::clone(&pair[1]), env);
                            }
                        }
                        _ => {
                            return Err(format!(
                                "Expected boolean predicate in cond, found {}",
                                expr.as_ref()
                            ));
                        }
                    },
                    Err(msg) => {
                        return Err(msg);
                    }
                }
            }
            _ => {
                return Err(format!(
                    "Expected pair in form (predicate value), found {}",
                    expr.as_ref()
                ));
            }
        }
    }
    Err(String::from(
        "Conditional never found a satisfied predicate",
    ))
}

// If needs to be a special form to allow one of the values to not be evaluated
fn s_if(args: &[Rc<Expression>], env: &mut Environment) -> Result<Rc<Expression>, String> {
    //If deals with a triple: one predicate followed by two values.
    if args.len() != 3 {
        return Err(format!("Special form \"if\" expects three arguments, {} were given", args.len()));
    }

    let pred = eval(Rc::clone(&args[0]), env)?;

    match pred.as_ref() {
        Expression::Boolean(b) => {
            if *b {
                Ok(eval(Rc::clone(&args[1]), env)?)
            } else {
                Ok(eval(Rc::clone(&args[2]), env)?)
            }
        },
        _ => Err(format!("Expected boolean predicate in if expression, found {}", pred.as_ref()))
    }
}
