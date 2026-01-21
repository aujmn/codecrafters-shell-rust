use std::{
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

use crate::env::check_exe_in_env_path;

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

pub fn type_handler(arg: &str) -> Result<TypeResult, Error> {
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

pub fn cd_handler(arg: &str) -> Result<PathBuf, Option<Error>> {
    let path = Path::new(arg).canonicalize().map_err(|e| match e.kind() {
        ErrorKind::NotFound | ErrorKind::InvalidInput => None,
        _ => Some(e),
    })?;
    std::fs::read_dir(&path).map_err(|e| match e.kind() {
        ErrorKind::NotFound | ErrorKind::NotADirectory => None,
        _ => Some(e),
    })?;
    Ok(path)
}
