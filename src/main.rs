use std::rc::Rc;

use rustyline::error::ReadlineError;
use rustyline::Editor;

//Not sure I'm using rust modules correctly
mod eval;
mod parse;
mod types;

use eval::*;
use parse::*;
use types::*;

fn main() {
    let mut ed = Editor::<()>::new();
    let env = get_default_env();

    loop {
        match ed.readline(">> ") {
            Ok(line) => {
                ed.add_history_entry(line.as_ref());

                match parse_repl_line(line) {
                    Ok(vec) => {
                        for expr in vec.into_iter() {
                            match eval(Rc::clone(&expr), &env) {
                                Ok(result) => {
                                    println!("{}", result);
                                }
                                Err(msg) => {
                                    println!("{}", msg);
                                }
                            }
                        }
                    }
                    Err(s) => {
                        println!("{}", s);
                        std::process::exit(1);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Encountered ^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Encountered EOF");
                break;
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}

fn get_default_env() -> Environment {
    [(
        String::from("pi"),
        Rc::new(Expression::from(std::f32::consts::PI)),
    )]
        .iter()
        .cloned()
        .collect()
}
