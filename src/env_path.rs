use std::ffi::OsString;
use std::os::unix::fs::PermissionsExt;

pub fn check_exe_in_env_path(exe: &str) -> Option<String> {
    if std::env::consts::FAMILY == "windows" {
        todo!() // how?
    }
    let env_path = get_env_path();
    for path in std::env::split_paths(&env_path) {
        let metadata = match path.join(exe).metadata() {
            Ok(metadata) => metadata,
            Err(err) => {
                let _ = err.kind(); // todo: handle specific errors
                continue;
            }
        };
        if metadata.is_file() && metadata.permissions().mode() & 0o111 != 0
        // todo: check only for user?
        {
            match path.to_str() {
                Some(p) => return Some(p.to_string()),
                None => continue,
            }
        }
    }
    None
}

fn get_env_path() -> OsString {
    match std::env::var_os("PATH") {
        Some(path) => path,
        None => todo!(),
    }
}
