use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Log_Level {
    debug,
    info,
    warning,
    error
}

impl FromStr for Log_Level {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "debug" => Ok(Log_Level::debug),
            "info" => Ok(Log_Level::info),
            "warning" => Ok(Log_Level::warning),
            "error" => Ok(Log_Level::error),
            _ => Err(())
        }
    }
}

pub struct Logger {
    pub level: Log_Level
}

impl Logger {
    pub fn write_line(&self, level: Log_Level, value: &str) {
        if &self.level == &level {
            println!("{}", value);
        }
    }
}
