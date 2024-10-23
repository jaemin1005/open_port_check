use crate::traits::command::CommandExecutor;

/**
 * kill_process, 명령어에 대한 성공 및 실패에 대한 결과마 추출
 */
pub fn kill_process(executor: &dyn CommandExecutor, pid: String) -> bool {
    if cfg!(target_os = "windows") {
        executor
            .execute_command("taskkill", &["/PID", &pid, "/F"])
            .is_ok()
    } else if cfg!(target_os = "macos") {
        executor.execute_command("kill", &["-9", &pid]).is_ok()
    } else if cfg!(target_os = "linux") {
        executor.execute_command("kill", &["-9", &pid]).is_ok()
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::command::CommandExecutor;

    const TEST_PID: &str = "1234";

    const PROCESS_WINDOW_RESULT: &str = "SUCCESS: The process with PID 1234 has been terminated.";
    const PROCESS_MAC_RESULT: &str = "Process 1234 has been terminated.";
    const PROCESS_ERROR: &str = "Failed to terminate process.";

    struct MockKillCommandExecutor;

    impl CommandExecutor for MockKillCommandExecutor {
        fn execute_command(&self, command: &str, args: &[&str]) -> Result<String, String> {
            match command {
                "taskkill" => {
                    if args.contains(&"/PID") && args.contains(&TEST_PID) {
                        Ok(PROCESS_WINDOW_RESULT.to_string())
                    } else {
                        Err(PROCESS_ERROR.to_string())
                    }
                }
                "kill" => {
                    if args.contains(&"1234") {
                        Ok(PROCESS_MAC_RESULT.to_string())
                    } else {
                        Err(PROCESS_ERROR.to_string())
                    }
                }
                _ => Err(PROCESS_ERROR.to_string()),
            }
        }
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_kill_process_macos() {
        let mock_executor = MockKillCommandExecutor;
        let result = kill_process(&mock_executor, TEST_PID.to_string());
        assert_eq!(result, true);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_kill_process_macos() {
        let mock_executor = MockKillCommandExecutor;
        let result = kill_process(&mock_executor, TEST_PID.to_string());
        assert_eq!(result, true);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_kill_process_windows() {
        let mock_executor = MockKillCommandExecutor;
        let result = kill_process(&mock_executor, TEST_PID.to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn test_kill_process_failure() {
        let mock_executor = MockKillCommandExecutor;
        let result = kill_process(&mock_executor, "9999".to_string());
        assert_eq!(result, false);
    }
}
