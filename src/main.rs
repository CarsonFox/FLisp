use std::rc::Rc;

use rustyline::error::ReadlineError;
use rustyline::Editor;

//Not sure I'm using rust modules correctly
mod eval;
mod parse;
mod types;

use eval::*;
use parse::*;

fn main() {
    let mut ed = Editor::<()>::new();
    let mut env = load_stdlib(include_bytes!("stdlib.scm"));

    loop {
        match ed.readline(">> ") {
            Ok(line) => {
                ed.add_history_entry(line.as_ref());

                match parse_repl_line(line) {
                    Ok(vec) => {
                        for expr in vec.into_iter() {
                            match eval(Rc::clone(&expr), &mut env) {
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
