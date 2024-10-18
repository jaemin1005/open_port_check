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
 * 윈도우의 명령어의 결과를 파싱하혀 Process, Port, Pid의 집합으로 나타낸다.
 */
fn parsing_window_netstat(output: &str) -> Vec<(String, String, String)> {
  output.lines().skip(4).filter_map(|line| {
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
}).collect()
}
