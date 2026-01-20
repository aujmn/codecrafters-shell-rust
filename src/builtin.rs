use crate::env_path::check_exe_in_env_path;

const BUILTIN_KEYWORDS: [&str; 3] = ["exit", "echo", "type"];

pub fn type_handler(arg: &str) -> String {
    if BUILTIN_KEYWORDS.contains(&arg)
    // todo: use a set when this program supports many keywords
    {
        format!("{arg} is a shell builtin")
    } else {
        match check_exe_in_env_path(arg) {
            Some(path) => {
                format!("{arg} is {path}{}{arg}", std::path::MAIN_SEPARATOR)
            }
            None => format!("{arg}: not found"),
        }
    }
}
