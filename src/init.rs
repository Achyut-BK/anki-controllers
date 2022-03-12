fn init_gamepad() -> Option<usize> {
    unimplemented!();
}

use log::{LevelFilter, SetLoggerError};

pub fn init_log(log_level: Option<LevelFilter>) -> Result<(), SetLoggerError> {
    let mut builder = env_logger::Builder::new();

    if let Some(level) = log_level {
        builder.filter_level(level);
    };
    builder.try_init()
}
