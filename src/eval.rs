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

fn apply(list: &Vec<Rc<Expression>>, env: &Environment) -> Result<Rc<Expression>, String> {
    //General approach: Look up the first expression. It could be a procedure, in which case
    //it just needs to be called. If the lookup fails, try to recognize a special form.

    if let Some(result) = special_form(list, env) {
        return result;
    }

    Err(String::from("Function application not supported."))
}

fn special_form(_list: &Vec<Rc<Expression>>, _env: &Environment) -> Option<Result<Rc<Expression>, String>> {
    //In order to write this, the first element of list needs to be passed back to eval. This means
    //either the elements need to be wrapped in Rc, or Rc's are made here.

    None
}
