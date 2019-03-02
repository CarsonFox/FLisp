use rustyline::error::ReadlineError;
use rustyline::Editor;

mod parse;
use parse::*;
mod types;
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
                        for expr in vec.iter() {
                            match eval(expr) {
                                Ok(atom) => println!("{}", atom),
                                Err(msg) => println!("{}", msg),
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
