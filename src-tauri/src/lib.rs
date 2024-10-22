use enums::os::OS;
use traits::command::OSCommandExecutor;
use utils::{kill, port::{self, parsing_mac_lsof, parsing_window_netstat}, remove_duplicate::remove_duplicates};

pub mod utils {
    pub mod port;
    pub mod kill;
    pub mod remove_duplicate;
}

pub mod traits {
    pub mod command;
}

pub mod enums {
    pub mod os;
}

#[tauri::command]
fn get_open_ports() -> Vec<(String, String, String)> {
    let executor = OSCommandExecutor;
    let os = port::get_open_ports(&executor);

    let result = match os {
        OS::MacOS(output) => parsing_mac_lsof(&output),
        OS::Windows(output) => parsing_window_netstat(&output),
        _ => panic!("Unsupported OS"),
    };

    remove_duplicates(result)
}

#[tauri::command]
fn kill_process(pid: &str) -> bool {
    let executor = OSCommandExecutor;
    kill::kill_process(&executor, pid.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_open_ports, kill_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
