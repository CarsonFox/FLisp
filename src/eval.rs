use crate::types::*;

pub fn eval(expr: &Expression, env: &Environment) -> Result<Atom, String> {
    match expr {
        Expression::Atomic(atom) => Ok(atom.clone()),
        Expression::SExpr(vec) => match eval(&vec[0], env) {
            Ok(proc) => apply(proc, &vec[1..], env),
            Err(msg) => Err(msg),
        },
    }
}

fn apply(proc: Atom, args: &[Expression], env: &Environment) -> Result<Atom, String> {
    match proc {
        Atom::Identifier(id) => {
            //Try to lookup the procedure
            match env.get(&id) {
                Some(proc) => {
                    //Evaluate arguments
                    let mut args_atomic = Vec::with_capacity(args.len());
                    for result in args.iter().map(|expr| eval(expr, env)) {
                        match result {
                            Ok(atom) => {
                                args_atomic.push(atom);
                            }
                            Err(msg) => {
                                return Err(msg);
                            }
                        }
                    }
                    match proc {
                        Procedure::Builtin(builtin) => builtin(&args[..]),
                    }
                }
                None => Err(format!("Undefined procedure: {}", id)),
            }
        }
        _ => Err(format!("Application of non-procedure object: {}", proc)),
    }
}
