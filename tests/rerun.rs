mod utils;

use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::Child;
use std::process::{Command, Stdio};
use std::str::from_utf8;
use std::thread::sleep;
use std::time::Duration;
use utils::shell_command;

#[test]
fn does_not_rerun_if_status_does_not_match() {
    let stdout = Command::new(env!("CARGO_BIN_EXE_prcs"))
        .arg("-r=100")
        .args(&shell_command("echo text"))
        .output()
        .unwrap()
        .stdout;
    assert_eq!(
        from_utf8(&stdout).unwrap(),
        if cfg!(windows) { "text\r\n" } else { "text\n" }
    )
}

const EXPECTED_OUTPUT: &'static str = if cfg!(windows) {
    "text\r\nEND\r\n"
} else {
    "text\nEND\n"
};
fn run_consume_and_print_one_stdin_line(command: &mut Command) -> Child {
    if cfg!(windows) {
        command.args([
            "cscript",
            "/nologo",
            Path::new(file!())
                .join("../utils/consume-and-print-one-stdin-line.js")
                .to_str()
                .unwrap(),
        ]);
    } else {
        command.args(&shell_command(
            "(read line && echo $line) || (echo END && false)",
        ));
    }

    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(if cfg!(windows) {
            b"text\r\n"
        } else {
            b"text\n"
        })
        .unwrap();
    child
}

#[test]
fn reruns_if_status_matches() {
    let mut command = Command::new(env!("CARGO_BIN_EXE_prcs"));
    command.arg("-r=0");

    let mut child = run_consume_and_print_one_stdin_line(&mut command);
    assert_eq!(child.wait().unwrap().code().unwrap(), 1);

    let mut output = vec![];
    child
        .stdout
        .take()
        .unwrap()
        .read_to_end(&mut output)
        .unwrap();

    assert_eq!(from_utf8(&output).unwrap(), EXPECTED_OUTPUT)
}

#[test]
fn reruns_before_stalling() {
    let mut command = Command::new(env!("CARGO_BIN_EXE_prcs"));
    command.arg("-r=0");
    command.arg("-s=1");

    let mut child = run_consume_and_print_one_stdin_line(&mut command);
    drop(child.stdin.take().unwrap());

    let mut output = [0; EXPECTED_OUTPUT.len()];
    child
        .stdout
        .take()
        .unwrap()
        .read_exact(&mut output)
        .unwrap();
    assert_eq!(from_utf8(&output).unwrap(), EXPECTED_OUTPUT);

    sleep(Duration::from_millis(100));
    assert!(child.try_wait().unwrap().is_none())
}
