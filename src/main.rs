extern crate nom;

extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod parse;
use parse::*;

mod eval;
use eval::*;

fn main() {
    let mut ed = Editor::<()>::new();
    loop {
        match ed.readline(">> ") {
            Ok(line) => {
                ed.add_history_entry(line.as_ref());

                match parse_repl_line(line) {
                    Ok(vec) => {
                        for e in vec.iter() {
                            eval(e);
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
