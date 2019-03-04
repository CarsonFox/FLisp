//use crate::types::*;
//use std::rc::Rc;
//
//pub fn get_default_env() -> Environment {
//    [(String::from("+"), Procedure::Builtin(add_exprs))]
//        .iter()
//        .cloned()
//        .collect()
//}
//
//fn add_exprs(args: Vec<Rc<Atom>>) -> Result<Rc<Atom>, String> {
//    if args.len() < 2 {
//        return Err(format!(
//            "Invalid number of arguments to procedure \"+\": {}",
//            args.len()
//        ));
//    }
//
//    if let Some(a) = args.iter().find(|a| !a.is_number()) {
//        return Err(format!("Attempted to add non-numeric object: {}", a));
//    }
//
//    Ok(Rc::new(args[1..].iter().map(|a| a.as_ref()).fold(
//        args[0].as_ref().clone(),
//        |acc, x| match acc {
//            Atom::Numeric(acc) => match x {
//                Atom::Numeric(x) => Atom::Numeric(acc + *x),
//                _ => unreachable!(),
//            },
//            _ => unreachable!(),
//        },
//    )))
//}
