use crate::types::*;

pub fn get_default_env() -> Environment {
    [(String::from("+"), Procedure::Builtin(add_exprs))]
        .iter()
        .cloned()
        .collect()
}

fn add_exprs(args: &[Atom]) -> Result<Atom, String> {
    if args.len() < 2 {
        return Err(format!(
            "Invalid number of arguments to procedure \"+\": {}",
            args.len()
        ));
    }

    for arg in args {
        match arg {
            Atom::Numeric(_) => {}
            _ => {
                return Err(format!("Cannot add to {}", arg));
            }
        }
    }

    Ok(args[1..].iter().fold(args[0].clone(), |acc, x| match acc {
        Atom::Numeric(acc) => match x {
            Atom::Numeric(x) => Atom::Numeric(acc + *x),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }))
}
