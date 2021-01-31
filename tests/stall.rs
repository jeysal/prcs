mod utils;

use std::io::Read;
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use utils::shell_command;

const SLEEP_DURATION: Duration = Duration::from_millis(10);
const SLEEP_ATTEMPS: u32 = 100;

#[test]
fn does_not_stall_if_status_does_not_match() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_prcs"))
        .arg("-s=>=100")
        .args(&shell_command("exit 42"))
        .spawn()
        .unwrap();

    for _ in 0..SLEEP_ATTEMPS {
        if child.try_wait().unwrap().is_some() {
            return;
        }
        sleep(SLEEP_DURATION);
    }
    panic!(
        "prcs did not exit in more than {:?}.",
        SLEEP_ATTEMPS * SLEEP_DURATION
    )
}

#[test]
fn stalls_if_status_matches() {
    const SLEEP_DURATION: Duration = Duration::from_millis(10);
    const SLEEP_ATTEMPS: u32 = 100;

    let mut child = Command::new(env!("CARGO_BIN_EXE_prcs"))
        .arg("-s=>=100")
        .args(&shell_command("echo text && exit 123"))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut child_stdout = child.stdout.take().unwrap();

    let mut output = [0; 4];
    for _ in 0..SLEEP_ATTEMPS {
        child_stdout.read(&mut output).unwrap();
        if &output == b"text" {
            sleep(SLEEP_DURATION);
            if child.try_wait().unwrap().is_none() {
                return;
            }
        }
        sleep(SLEEP_DURATION);
    }
    panic!(
        "Did not observe child command exit in more than {:?}.",
        SLEEP_ATTEMPS * SLEEP_DURATION
    )
}
