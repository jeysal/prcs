pub const fn shell_command(command: &'static str) -> [&'static str; 4] {
    if cfg!(windows) {
        ["--", "cmd", "/C", command]
    } else {
        ["--", "sh", "-c", command]
    }
}
