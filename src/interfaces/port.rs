use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PortInfo(String, String, String);

impl PortInfo {
    pub fn get_process_name(&self) -> String {
        self.0.clone()
    }

    pub fn get_port(&self) -> String {
        self.1.clone()
    }

    pub fn get_pid(&self) -> String {
        self.2.clone()
    }

    pub fn get_port_as_usize(&self) -> usize {
        self.1.parse::<usize>().unwrap()
    }

    pub fn get_pid_as_usize(&self) -> usize {
        self.2.parse::<usize>().unwrap()
    }
}
