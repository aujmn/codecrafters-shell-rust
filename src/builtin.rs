use std::{
    io::{ErrorKind, Result},
    path::PathBuf,
};

use crate::env::{check_exe_in_env_path, get_env_home};

// use a set or enum when this program supports many keywords
const BUILTIN_KEYWORDS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

pub enum TypeResult {
    Builtin(String),
    Executable {
        command: PathBuf,
        path_to_command: PathBuf,
    },
    Unknown(String),
}

impl std::fmt::Display for TypeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeResult::Builtin(name) => write!(f, "{name} is a shell builtin"),
            TypeResult::Executable {
                command,
                path_to_command,
            } => write!(
                f,
                "{} is {}{}{}",
                command.display(),
                path_to_command.display(),
                std::path::MAIN_SEPARATOR,
                command.display()
            ),
            TypeResult::Unknown(name) => write!(f, "{name}: not found"),
        }
    }
}

pub fn type_handler(arg: &str) -> Result<TypeResult> {
    if BUILTIN_KEYWORDS.contains(&arg) {
        Ok(TypeResult::Builtin(arg.to_string()))
    } else {
        match check_exe_in_env_path(arg)? {
            Some(path) => Ok(TypeResult::Executable {
                command: arg.into(),
                path_to_command: path,
            }),
            None => Ok(TypeResult::Unknown(arg.to_string())),
        }
    }
}

pub fn cd_handler(arg: &str, current_dir: &mut PathBuf) -> Result<()> {
    if arg == "~" {
        *current_dir = get_env_home()?.into();
        return Ok(());
    }
    current_dir.join(arg).canonicalize().map_or_else(
        |e| match e.kind() {
            ErrorKind::NotFound | ErrorKind::InvalidInput => {
                println!("cd: {}: No such file or directory", arg);
                Ok(())
            }
            _ => Err(e),
        },
        |path|
            // open the directory to check; or use `path.is_dir()`?
            std::fs::read_dir(&path).map_or_else(
                |e| match e.kind() {
                    ErrorKind::NotADirectory => {
                        println!("cd: {}: No such file or directory", arg);
                        Ok(())
                    }
                    _ => Err(e),
                },
                |_| {
                    *current_dir = path;
                    Ok(())
                },
            ),
    )
}
