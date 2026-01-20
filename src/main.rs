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
                debug_assert!(args.next() == None); // todo: handle exit command with args
                break;
            }
            "echo" => {
                println!("{}", args.collect::<Vec<&str>>().join(" "));
                continue;
            }
            "type" => {
                let arg = args.next().unwrap(); // todo: handle type command without exactly one arg
                println!("{}", type_handler(arg));
                continue;
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
                            .spawn()?
                            .wait()?; // todo: any other approach?
                    }
                    Unknown(_) => println!("{input}: command not found"),
                };
            }
        }
    }
    Ok(())
}
