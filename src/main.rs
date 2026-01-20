mod builtin;
mod env_path;

use builtin::{TypeResult::*, type_handler};
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();

        let mut args = input.split_whitespace();
        let Some(command) = args.next() else {
            continue; // ignore empty input
        };
        match command {
            "exit" => {
                debug_assert!(args.next().is_none()); // todo: handle exit command with args
                break;
            }
            "echo" => {
                println!("{}", args.collect::<Vec<&str>>().join(" "));
            }
            "type" => {
                let arg = args.next().unwrap(); // todo: handle type command without exactly one arg
                println!("{}", type_handler(arg));
            }
            "pwd" => {
                let dir = std::env::current_dir()?.canonicalize()?; // todo: handle errors
                println!("{}", dir.display());
            }
            _ => {
                match type_handler(command) {
                    Builtin(_) => unreachable!(),
                    Executable {
                        command,
                        path_to_command: _,
                    } => {
                        let args = args.collect::<Vec<&str>>();
                        std::process::Command::new(command)
                            .args(args)
                            .spawn()? // todo: handle errors
                            .wait()?; // todo: any other approach besides waiting?
                    }
                    Unknown(_) => println!("{input}: command not found"),
                };
            }
        }
    }
    Ok(())
}
