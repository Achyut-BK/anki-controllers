use gilrs::ev::{Button, EventType};
use gilrs::{Event, Gilrs, MappingSource};
use log::{info, warn};

pub fn init_gamepad() -> Option<usize> {
    let mut gilrs = Gilrs::new().unwrap();
    let mut active_gamepad_id: Option<usize>;

    loop {
        if let Some(Event {
            id,
            event: EventType::ButtonPressed(Button::Mode, _),
            time: _,
        }) = gilrs.next_event()
        {
            match gilrs.gamepad(id).mapping_source() {
                MappingSource::None => {
                    warn!("This Gamepad is Unsupported. It may still work. Use at own Risk")
                }
                _ => info!("{} is supported, Proceeding", gilrs.gamepad(id).name()),
            }
            active_gamepad_id = Some(id.into());
            break;
        }
    }
    active_gamepad_id
}

use log::{LevelFilter, SetLoggerError};

pub fn init_log(log_level: Option<LevelFilter>) -> Result<(), SetLoggerError> {
    let mut builder = env_logger::Builder::new();

    if let Some(level) = log_level {
        builder.filter_level(level);
    };
    builder.try_init()
}
