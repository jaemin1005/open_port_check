use std::process::{Command, Stdio};
pub struct OSCommandExecutor;

pub trait CommandExecutor {
    fn execute_command(&self, command: &str, args: &[&str]) -> Result<String, String>;
}

impl CommandExecutor for OSCommandExecutor {
    fn execute_command(&self, command: &str, args: &[&str]) -> Result<String, String> {
        let output = Command::new(command)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .and_then(|child| child.wait_with_output())  // 프로세스 종료까지 기다림
            .map_err(|e| format!("Failed to execute command: {}", e))?;  // 에러 처리

        let result = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(result)
    }
}
