use std::process::{Command,Stdio};

fn main() {
    let status = Command::new("rustc")
        .arg("-V")
        .status() // this blocks until the process is finished
        .expect("Could not run that program");
    println!("cool {:?} code {:?}", status.success(), status.code().unwrap());

    // capturing output
    let output = Command::new("ls")
        .arg("-oa")
        .output() // this also blocks until the process is finished, and puts a Vec<u8> into `output.stdout`
        .expect("Cannot feel around here.");
    if output.status.success() {
        println!("Ok!");
    }
    println!("length of stdout: {}, length of stderr: {}", output.stdout.len(), output.stderr.len());
    println!("{}", String::from_utf8_lossy(&output.stdout).trim_right().to_string());

    // creating a child process that runs in the shell
    let shell_return = shell_success("ls");
    match shell_return {
        Some(output) => println!("{}", output),
        None => println!("An error occurred.")
    }

    let mut child = Command::new("rustc")
        .stdout(Stdio::null()) //suppress output
        .stderr(Stdio::null()) // stdio::piped() allows us to watch the child processes stdout/stderr using child.stdout
        .spawn()               // do not block, child process runs in background of the main thread
        .expect("no rustc?? how is this running?");
    let res = child.wait();    // rejoins the child process to the main thread. `wait_with_output()` allows for the Vec<u8> capture as `output()` can also `kill()` a child process
    println!("res: {:?}", res);
}

// running a command in the shell - whooo portable
fn shell_cmd(cmd: &str) -> (String, bool) {
    let cmd = format!("{} 2>&1", cmd);
    let output = Command::new(if cfg!(windows) { "cmd.exe" } else { "/bin/sh" })
        .arg(if cfg!(windows) { "/c" } else { "-c" })
        .arg(cmd)
        .output()
        .expect("where is my shell?");
    ( String::from_utf8_lossy(&output.stdout).trim_right().to_string(), output.status.success() )
}
fn shell_success(cmd: &str) -> Option<String> {
    let (output,success) = shell_cmd(cmd);
    if success {Some(output)} else {None}
}
