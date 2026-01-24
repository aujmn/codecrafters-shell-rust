use std::io::{Error, ErrorKind, Result};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub(crate) fn check_exe_in_env_path(exe: &str) -> Result<Option<PathBuf>> {
    if std::env::consts::FAMILY == "windows" {
        return Err(Error::other(
            "Windows does not support (UNIX-style) permission checking",
        ));
    }
    let env_path = std::env::var_os("PATH").ok_or(Error::other("Cannot get PATH"))?;
    for path in std::env::split_paths(&env_path) {
        let metadata = match path.join(exe).metadata() {
            Ok(metadata) => metadata,
            Err(err) => match err.kind() {
                ErrorKind::NotFound => continue,
                ErrorKind::PermissionDenied => return Err(err),
                _ => continue,
            },
        };
        if metadata.is_file() && metadata.permissions().mode() & 0o111 != 0
        // any executable permission for owner, group or others
        {
            return Ok(Some(path)); // first match in PATH
        }
    }
    Ok(None)
}

pub(crate) fn get_env_home() -> Result<std::ffi::OsString> {
    std::env::var_os("HOME").ok_or(Error::other("Cannot get HOME"))
}
