mod builtin;
mod env_path;

use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();

        match input.split_once(' ') {
            None => {
                if input == "exit" {
                    break;
                };
            }
            Some((command, args)) => match command {
                "echo" => {
                    if args.contains("  ") {
                        todo!() // handle multiple whitespaces
                    }
                    println!("{args}");
                    continue;
                }
                "type" => {
                    if args.contains(' ') {
                        todo!() // handle more than one argument
                    }
                    println!("{}", builtin::type_handler(args));
                    continue;
                }
                _ => {}
            },
        }

        println!("{input}: command not found");
    }
    Ok(())
}
