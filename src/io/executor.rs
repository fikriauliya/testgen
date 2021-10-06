use std::io::Write;
use std::process::{Command, Stdio};

pub fn execute(solution_command: &str, input: &str) -> String {
    let args = shlex::split(solution_command).unwrap();
    let mut cmd = Command::new(&args[0]);
    let mut child = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .args(args.iter().skip(1))
        .spawn()
        .expect("Failed to execute solution");
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    let stdout = output.stdout;
    let stdout = String::from_utf8(stdout).unwrap();
    // let stderr = output.stderr;
    // TODO: handle stderr
    // let stderr = String::from_utf8(stderr).unwrap();
    stdout
}
