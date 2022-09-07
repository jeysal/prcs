mod utils;

use std::fs::read_to_string;
use std::io::Write;
use std::process::{Command, Stdio};
use std::str::from_utf8;

use tempfile::tempdir;
use utils::shell_command;

#[test]
fn forwards_status_code() {
    let status_code = Command::new(env!("CARGO_BIN_EXE_prcs"))
        .args(&shell_command("exit 42"))
        .status()
        .unwrap()
        .code()
        .unwrap();
    assert_eq!(status_code, 42)
}

#[test]
fn forwards_stdout() {
    let stdout = Command::new(env!("CARGO_BIN_EXE_prcs"))
        .args(&shell_command("echo text"))
        .output()
        .unwrap()
        .stdout;
    assert_eq!(
        from_utf8(&stdout).unwrap(),
        if cfg!(windows) { "text\r\n" } else { "text\n" }
    )
}

#[test]
fn forwards_stderr() {
    let stderr = Command::new(env!("CARGO_BIN_EXE_prcs"))
        .args(&shell_command("echo text 1>&2"))
        .output()
        .unwrap()
        .stderr;
    assert_eq!(
        from_utf8(&stderr).unwrap(),
        if cfg!(windows) { "text \r\n" } else { "text\n" }
    )
}

#[test]
fn forwards_stdin() {
    let tmp_dir = tempdir().unwrap();
    let mut tmp_file_path = tmp_dir.path().to_path_buf();
    tmp_file_path.push("file.txt");

    let mut child = Command::new(env!("CARGO_BIN_EXE_prcs"))
        .args(&shell_command(if cfg!(windows) {
            "more >%TMP_FILE_PATH%"
        } else {
            "cat >$TMP_FILE_PATH"
        }))
        .env("TMP_FILE_PATH", tmp_file_path.to_str().unwrap())
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.as_mut().unwrap().write_all(b"text").unwrap();
    assert_eq!(child.wait().unwrap().code().unwrap(), 0);

    if cfg!(windows) {
        // starts_with because there may or may not be a line break added on Windows
        assert!(read_to_string(tmp_file_path).unwrap().starts_with("text"));
    } else {
        assert_eq!(read_to_string(tmp_file_path).unwrap(), "text");
    }
}

#[test]
fn fails_if_command_invalid() {
    let status_code = Command::new(env!("CARGO_BIN_EXE_prcs"))
        .arg("executable-that-almost-certainly-does-not-exist")
        .stderr(Stdio::null())
        .status()
        .unwrap()
        .code()
        .unwrap();
    assert_ne!(status_code, 0)
}
#[test]
fn prints_error_if_command_invalid() {
    let stderr = Command::new(env!("CARGO_BIN_EXE_prcs"))
        .arg("executable-that-almost-certainly-does-not-exist")
        .output()
        .unwrap()
        .stderr;
    let error_message = from_utf8(&stderr).unwrap();
    assert!(error_message.to_lowercase().contains("failed to run"));
    assert!(error_message.contains("executable-that-almost-certainly-does-not-exist"));
}
