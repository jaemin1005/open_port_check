use std::process::Command;
pub struct OSCommandExecutor;

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
