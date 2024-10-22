use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KillArgs<'a> {
    pub pid: &'a str,
}