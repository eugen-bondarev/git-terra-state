use std::process::Command;

pub fn run_command(cmd: String) {
    // println!("{}", cmd);
    Command::new("sh").arg("-c").arg(cmd).output().unwrap();
    // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
