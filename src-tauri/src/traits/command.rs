use std::process::Command;

pub struct OSCommandExecutor;
pub struct MockCommandExecutor;

pub trait CommandExecutor {
    fn execute_command(&self, command: &str, args: &[&str]) -> Result<String, String>;
}

impl CommandExecutor for OSCommandExecutor {
    fn execute_command(&self, command: &str, args: &[&str]) -> Result<String, String> {
        let output = Command::new(command)
            .args(args)
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        let result = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(result)
    }
}

impl CommandExecutor for MockCommandExecutor {
    fn execute_command(&self, command: &str, _args: &[&str]) -> Result<String, String> {
        match command {
            "netstat" => Ok("Proto  Local Address          State           PID\nTCP    127.0.0.1:8080    LISTENING       1234".to_string()),
            "lsof" => Ok("COMMAND   PID   USER   NODE NAME\nchrome    5678  user   0t0  TCP 192.168.1.10:3000 (LISTEN)".to_string()),
            _ => Err("Unsupported command".to_string()),
        }
    }
}
