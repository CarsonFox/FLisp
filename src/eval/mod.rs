use crate::parse::*;

pub fn eval(expr: &Expression) -> Atom {
    match expr {
        Expression::Atomic(a) => a.clone(),
        Expression::SExpr(vec) => apply(eval(&vec[0]), &vec[1..]),
    }
}

fn apply(proc: Atom, args: &[Expression]) -> Atom {
    Atom::Numeric(Number::Integer(0))
}
