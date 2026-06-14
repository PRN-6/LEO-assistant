use std::process::Command;

pub fn launch(cmd_line: &str) {
    // Use the shell to run the command so paths with spaces and special chars work correctly
    let _ = Command::new("sh")
        .arg("-c")
        .arg(cmd_line)
        .spawn();
}
