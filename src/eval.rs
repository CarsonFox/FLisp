use crate::types::*;

//This is almost certainly going to change.
pub type Environment = std::collections::HashMap<String, fn(&[Atom]) -> Result<Atom, String>>;

pub fn eval(expr: &Expression) -> Result<Atom, String> {
    match expr {
        Expression::Atomic(atom) => Ok(atom.clone()),
        Expression::SExpr(vec) => {
            match eval(&vec[0]) {
                Ok(proc) => apply(proc, &vec[1..]),
                Err(msg) => Err(msg),
            }
        }
    }
}

fn apply(proc: Atom, args: &[Expression]) -> Result<Atom, String> {
    Ok(Atom::Integer(0))
}

//fn add_exprs(_args: &[Atom]) -> Result<Atom, String> {
//    Ok(Atom::Integer(0))
//}