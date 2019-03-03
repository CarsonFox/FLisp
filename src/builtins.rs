//use crate::types::*;
//
//pub fn get_default_env() -> Environment {
//    [(String::from("+"), Procedure::Builtin(add_exprs))]
//        .iter()
//        .cloned()
//        .collect()
//}
//
//fn add_exprs(args: &[Atom]) -> Result<Atom, String> {
//    if args.len() < 2 {
//        return Err(format!(
//            "Invalid number of arguments to procedure \"+\": {}",
//            args.len()
//        ));
//    }
//
//    Ok(args[1..]
//        .iter()
//        .fold(args[0].clone(), |acc, other| match acc {
//            Atom::Integer(x) => match other {
//                Atom::Integer(y) => Atom::Integer(x + *y),
//                Atom::Float(y) => Atom::Float(x as f32 + *y),
//                Atom::Identifier(_id) => unimplemented!(),
//            },
//            Atom::Float(x) => match other {
//                Atom::Integer(y) => Atom::Float(x + *y as f32),
//                Atom::Float(y) => Atom::Float(x + *y),
//                Atom::Identifier(_id) => unimplemented!(),
//            },
//            Atom::Identifier(_id) => unimplemented!(),
//        }))
//}
