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

    if let Some(a) = args.iter().find(|a| !a.is_number()) {
        return Err(format!("Attempted to add non-numeric object: {}", a));
    }

    Ok(args[1..].iter().fold(args[0].clone(), |acc, x| match acc {
        Atom::Numeric(acc) => match x {
            Atom::Numeric(x) => Atom::Numeric(acc + *x),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }))
}
