use std::{
    io::{self},
    path::PathBuf,
};

use crate::builtin::{TypeResult::*, cd_handler, type_handler};

pub(crate) fn switcher<'a>(
    args: &mut core::slice::Iter<'a, String>,
    current_dir: &mut PathBuf,
) -> Result<Option<()>, SwitcherError<'a>> {
    let Some(command) = args.next() else {
        return Ok(Some(())); // ignore empty input
    };
    match command.as_str() {
        "exit" => {
            return if args.next().is_some() {
                Err(SwitcherError::Args { command, count: 0 })
            } else {
                Ok(None) // exit signal
            };
        }
        "echo" => {
            println!(
                "{}",
                args.map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            );
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
            cd_handler(arg.unwrap(), current_dir)?;
        }
        _ =>
        // "exec"; todo: extract into a handler?
        {
            match type_handler(command)? {
                Builtin(_) => unreachable!(),
                Executable {
                    command,
                    path_to_command: _,
                } => {
                    let args = args.map(|arg| arg.to_string()).collect::<Vec<_>>();
                    std::process::Command::new(command)
                        .args(args)
                        .spawn()?
                        .wait()?;
                }
                Unknown(_) => println!("{command}: command not found"),
            };
        }
    };
    Ok(Some(()))
}

pub(crate) enum SwitcherError<'a> {
    Args { command: &'a str, count: u8 },
    Io(io::Error),
}

impl<'a> From<io::Error> for SwitcherError<'a> {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

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
