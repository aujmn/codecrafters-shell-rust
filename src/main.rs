mod builtin;
mod env;

use builtin::{TypeResult::*, cd_handler, type_handler};
use std::{
    io::{self, Write},
    path::PathBuf,
};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut current_dir = std::env::current_dir()?.canonicalize()?; // shouldn't start up

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
        match switcher(command, &mut args, &mut current_dir, &input) {
            Ok(Some(())) => break Ok(()),
            Ok(None) => {}
            Err(e) => eprintln!("{e}"),
        }
    }
}

fn switcher<'a>(
    command: &'a str,
    args: &mut std::str::SplitWhitespace<'_>,
    current_dir: &mut PathBuf,
    input: &String,
) -> Result<Option<()>, SwitcherError<'a>> {
    match command {
        "exit" => {
            return if args.next().is_some() {
                Err(SwitcherError::Args { command, count: 0 })
            } else {
                Ok(Some(())) // exit signal
            };
        }
        "echo" => {
            println!("{}", args.collect::<Vec<&str>>().join(" "));
        }
        "type" => {
            let arg = args.next();
            if arg.is_none() || args.next().is_some() {
                return Err(SwitcherError::Args { command, count: 1 });
            }
            let result = type_handler(arg.unwrap())?;
            println!("{result}");
        }
        "pwd" => {
            println!("{}", current_dir.display());
        }
        "cd" => {
            let arg = args.next();
            if arg.is_none() || args.next().is_some() {
                return Err(SwitcherError::Args { command, count: 1 });
            }
            let arg = arg.unwrap();
            if let Some(e) = cd_handler(arg, current_dir) {
                return Err(e.into());
            }
        }
        _ =>
        // exec
        {
            match type_handler(command)? {
                Builtin(_) => unreachable!(),
                Executable {
                    command,
                    path_to_command: _,
                } => {
                    let args = args.collect::<Vec<&str>>();
                    std::process::Command::new(command)
                        .args(args)
                        .spawn()?
                        .wait()?;
                }
                Unknown(_) => println!("{input}: command not found"),
            };
        }
    }
    Ok(None)
}

enum SwitcherError<'a> {
    Args { command: &'a str, count: u8 },
    Io(io::Error),
}

impl<'a> From<io::Error> for SwitcherError<'a> {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

// impl<'a> std::error::Error for SwitcherError<'a> {}

impl<'a> std::fmt::Display for SwitcherError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SwitcherError::Args { command, count } => {
                write!(
                    f,
                    "{}: requires {} {}",
                    command,
                    count,
                    if *count == 1 { "argument" } else { "arguments" }
                )
            }
            SwitcherError::Io(e) => e.fmt(f),
        }
    }
}
