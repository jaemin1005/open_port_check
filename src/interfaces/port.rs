use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct PortInfo(String, String, String);