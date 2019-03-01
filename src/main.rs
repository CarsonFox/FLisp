extern crate nom;

extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod parse;
use parse::*;

fn main() {
    let mut ed = Editor::<()>::new();
    loop {
        match ed.readline(">> ") {
            Ok(line) => {
                ed.add_history_entry(line.as_ref());
                let _ = dbg!(parse_repl_line(line));
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
