use std::{fmt::Display, path::PathBuf};

use crate::env_path::check_exe_in_env_path;

const BUILTIN_KEYWORDS: [&str; 3] = ["exit", "echo", "type"];

// pub enum Builtin {
//     Exit,
//     Echo,
//     Type,
// }

// impl Display for Builtin {
//     todo!(); // use a third-party crate to help?
// }

// impl TryFrom<&str> for Builtin {
//     todo!();
// }

pub enum TypeResult {
    Builtin(String),
    Executable {
        command: PathBuf,
        path_to_command: PathBuf,
    },
    Unknown(String),
}

impl Display for TypeResult {
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

pub fn type_handler(arg: &str) -> TypeResult {
    if BUILTIN_KEYWORDS.contains(&arg)
    // todo: use a set when this program supports many keywords
    {
        TypeResult::Builtin(arg.to_string())
    } else {
        match check_exe_in_env_path(&arg) {
            Some(path) => TypeResult::Executable {
                command: arg.into(),
                path_to_command: path,
            },
            None => TypeResult::Unknown(arg.to_string()),
        }
    }
}
