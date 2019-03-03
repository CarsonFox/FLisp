use rustyline::error::ReadlineError;
use rustyline::Editor;

mod parse;
use parse::*;
mod eval;
mod types;
use eval::*;
mod builtins;
use builtins::*;

fn main() {
    let mut ed = Editor::<()>::new();
//    let env = get_default_env();

    loop {
        match ed.readline(">> ") {
            Ok(line) => {
                ed.add_history_entry(line.as_ref());

                match parse_repl_line(line) {
                    Ok(vec) => {
                        for expr in vec.iter() {
                            let _ = dbg!(expr);
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
