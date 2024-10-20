#[derive(Clone)]
pub enum FILTER {
    PROCESS,
    PORT,
    PID,
}

impl ToString for FILTER {
    fn to_string(&self) -> String {
        match self {
            FILTER::PROCESS => "PROCESS".to_string(),
            FILTER::PORT => "PORT".to_string(),
            FILTER::PID => "PID".to_string(),
        }
    }
}

impl std::str::FromStr for FILTER {
    type Err = ();
    fn from_str(input: &str) -> Result<FILTER, Self::Err> {
        match input {
            "PROCESS" => Ok(FILTER::PROCESS),
            "PORT" => Ok(FILTER::PORT),
            "PID" => Ok(FILTER::PID),
            _ => Err(()),
        }
    }
}