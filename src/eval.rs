use crate::types::*;
use std::collections::HashSet;
use std::rc::Rc;

use lazy_static::lazy_static;

lazy_static! {
    static ref SPECIAL_FORMS: HashSet<&'static str> = ["+"].iter().cloned().collect();
}

pub fn eval(expr: Rc<Expression>, env: &Environment) -> Result<Rc<Expression>, String> {
    match expr.as_ref() {
        Expression::Numeric(_) => Ok(Rc::clone(&expr)),
        Expression::Identifier(id) => match env.get(id) {
            Some(expr) => Ok(Rc::clone(expr)),
            None => {
                if is_special_form(expr.as_ref()) {
                    Ok(expr)
                } else {
                    Err(format!("Unbound variable: {}", id))
                }
            },
        },
        Expression::SExpr(list) => apply(list, env),
    }
}

fn apply(list: &Vec<Rc<Expression>>, env: &Environment) -> Result<Rc<Expression>, String> {
    //General approach: Try to eval first sub-expression. If it can't be evaluated, check to see
    //if it's a special form.
    if list.is_empty() {
        return Err(String::from("Empty application"));
    }

    let result = eval(Rc::clone(&list[0]), env);

    if let Err(msg) = result {
        return Err(msg);
    }

    match result.unwrap().as_ref() {
        Expression::Numeric(_) => return Err(String::from("Cannot apply Number as a Procedure.")),
        Expression::Identifier(id) => {
            if let Some(expr) = env.get(id) {
                return call(Rc::clone(expr), &list[1..], env);
            } else if let Some(result) = special_form(list, env) {
                return result;
            } else {
                return Err(format!("Unrecognized procedure: {}", id));
            }
        },
        _ => unreachable!()
    }
}

fn call(proc: Rc<Expression>, _list: &[Rc<Expression>], _env: &Environment) -> Result<Rc<Expression>, String> {
    return Err(format!("Applying procedure: {}", proc.as_ref()));
}

//Check for a special form. Returns None if no special form was found, unless an error occurs.
fn special_form(_list: &Vec<Rc<Expression>>, _env: &Environment) -> Option<Result<Rc<Expression>, String>> {
//    let result = eval(Rc::clone(&list[0]), env);
//
//    match result {
//        Ok(expr) => {
//            match expr.as_ref() {
//                Expression::SExpr(_) |
//                Expression::Numeric(_) => return None,
//                _ => {}
//            }
//        },
//        Err(msg) => return Some(Err(msg)),
//    }

    None
}

fn is_special_form(expr: &Expression) -> bool {
    match expr {
        Expression::Identifier(id) => SPECIAL_FORMS.contains(id.as_str()),
        _ => false,
    }
}
