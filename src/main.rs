mod builtin;
mod control;
mod env;

use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut input = String::with_capacity(4096);
    let mut current_dir = std::env::current_dir()?.canonicalize()?; // shouldn't start up

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();
        let input_parsed = match control::parser(&input) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };

        match control::switcher(&mut input_parsed.iter(), &mut current_dir) {
            Ok(Some(())) => {}
            Ok(None) => break Ok(()),
            Err(e) => eprintln!("{e}"),
        }
    }
}
