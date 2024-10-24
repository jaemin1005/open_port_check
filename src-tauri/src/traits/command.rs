use std::process::{Command, Stdio};
pub struct OSCommandExecutor;

pub trait CommandExecutor {
    fn execute_command(&self, command: &str, args: &[&str]) -> Result<String, String>;
}

impl CommandExecutor for OSCommandExecutor {
    fn execute_command(&self, command: &str, args: &[&str]) -> Result<String, String> {
        let mut cmd = Command::new(command);
        cmd.args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // 윈도우 환경에서 특정 플래그를 설정하여 창이 안나오게 설정
        // FLAG => CREATE_NO_WINDOW
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x08000000);
        }

        let output = cmd
            .spawn()
            .and_then(|child| child.wait_with_output())
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        let result = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(result)
    }
}
