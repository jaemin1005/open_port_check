use std::{process::Command, result};

/**
 * 열려있는 Port 확인
 * os에 따라 열려있는 명령어를 다르게 호출한다.
 */
pub fn get_open_ports() -> String {
    if cfg!(target_os = "windows") {
        let output = Command::new("netstat")
            .arg("-aon")
            .output()
            .expect("failed to execute netstat");

        String::from_utf8_lossy(&output.stdout).to_string()
    } else if cfg!(target_os = "macos") {
        let output = Command::new("lsof")
            .arg("-i")
            .arg("-P")
            .arg("-n")
            .output()
            .expect("failed to execute lsof");

        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        "Unsupported operating system".to_string()
    }
}

/**
 * 윈도우의 명령어의 결과를 파싱하혀 (Process, Port, Pid)의 튜플로 나타낸다.
 *
 * Proto  Local Address          Foreign Address        State           PID
 * TCP    127.0.0.1:3000         0.0.0.0:0              LISTENING       1234
 */
fn parsing_window_netstat(output: &str) -> Vec<(String, String, String)> {
    output
        .lines()
        .skip(4)
        .filter_map(|line| {
            let columns: Vec<&str> = line.split_whitespace().collect();

            if columns.len() >= 5 && columns[3] == "LISTENING" {
                let local_address = columns[1].to_string();
                let pid = columns[4].to_string();

                if let Some(pos) = local_address.rfind(':') {
                    let port = &local_address[pos + 1..];
                    return Some(("Unknown".to_string(), port.to_string(), pid));
                }
            }
            None
        })
        .collect()
}

/**
 * 맥의 lsof의 결과를 파싱하여 (Process, Port, Pid)의 튜플로 나타낸다
 *
 * ex)
 * COMMAND   PID   USER   FD   TYPE    DEVICE SIZE/OFF NODE NAME
 * firefox   1234  user   45u  IPv4 0x1a2b3c 0t0      TCP 127.0.0.1:3000 (LISTEN)
 */
fn parsing_mac_lsof(output: &str) -> Vec<(String, String, String)> {
    output
        .lines()
        .skip(1)
        .filter_map(|line| {
            let columns: Vec<&str> = line.split_whitespace().collect();

            if columns.len() >= 9 && columns[9].contains("LISTEN") {
                let process = columns[0].to_string(); // 프로세스 이름 (COMMAND)
                let pid = columns[1].to_string(); // PID
                let port_info = columns[8]; // 포트 정보 (127.0.0.1:3000)

                // 포트 번호는 ":" 이후에 있음
                if let Some(pos) = port_info.rfind(':') {
                  let port = &port_info[pos + 1..];
                  return Some((process, port.to_string(), pid));
                }
            }
            None
        })
        .collect()
}

fn window_get_process_name(pid: &str) -> Option<String> {
    let output = Command::new("tasklist")
        .arg("/FI")
        .arg(format!("PID eq {}", pid))
        .output()
        .expect("failed to execute tasklist");

    let result = String::from_utf8_lossy(&output.stdout);

    // tasklist 결과에서 프로세스 이름 파싱
    for line in result.lines().skip(3) {
        let columns: Vec<&str> = line.split_whitespace().collect();
        if !columns.is_empty() && columns[1] == pid {
            return Some(columns[0].to_string()); // 프로세스 이름
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_mac_lsof() {
        // 가짜 lsof 명령어 결과
        let lsof_output = "\
COMMAND   PID   USER   FD   TYPE    DEVICE SIZE/OFF NODE NAME
firefox   1234  user   45u  IPv4 0x1a2b3c 0t0      TCP 127.0.0.1:3000 (LISTEN)
chrome    5678  user   48u  IPv4 0x4d5e6f 0t0      TCP 127.0.0.1:8080 (LISTEN)
";

        let expected_result = vec![
            (
                "firefox".to_string(),
                "3000".to_string(),
                "1234".to_string(),
            ),
            ("chrome".to_string(), "8080".to_string(), "5678".to_string()),
        ];

        let result = parsing_mac_lsof(lsof_output);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_parsing_mac_lsof_with_empty_output() {
        let empty_output = "";
        let expected_result: Vec<(String, String, String)> = vec![];
        let result = parsing_mac_lsof(empty_output);

        assert_eq!(result, expected_result);
    }
}
