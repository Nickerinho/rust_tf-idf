use crate::log::{Log_Level, Logger};

// defines only public methods to be called by cli / some other code
pub fn run(path: std::path::PathBuf, level: Log_Level) {
    // init logger
    let logger = Logger {
        level,
    };
    logger.write_line(Log_Level::error, "test_Logger");

}