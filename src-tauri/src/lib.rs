use enums::os::OS;
use traits::command::OSCommandExecutor;
use utils::port::{self, parsing_mac_lsof, parsing_window_netstat};

pub mod utils {
    pub mod port;
}

pub mod traits {
    pub mod command;
}

pub mod enums {
    pub mod os;
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_open_ports() -> Vec<(String, String, String)> {
    let executor = OSCommandExecutor;
    let os = port::get_open_ports(&executor);

    match os {
        OS::MacOS(output) => parsing_mac_lsof(&output),
        OS::Windows(output) => parsing_window_netstat(&output),
        _ => panic!("Unsupported OS"),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_open_ports])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
