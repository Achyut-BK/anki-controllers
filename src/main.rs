use env_logger::Builder;
use gilrs::ev::{Button, EventType};
use gilrs::{Event, Gilrs, MappingSource};
use log::LevelFilter;
use log::{error, info, warn};
use std::env;

fn main() {
    //  Initialize logger
    env::set_var("RUST_LOG", "Info");
    env_logger::init();
    // Create new gamepad access object
    let mut gilrs = Gilrs::new().unwrap();
    // Create active gamepad id
    let mut active_gamepad_id: Option<usize>;

    // Wait for Mode button press, test gamepad validity, and register gamepad
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
    //    let mut active_gamepad = None;
}

/*
Main
|
|- Wait for anki and Controllers to be connected - Init.rs
|- wait for controller input - Controller.rs
|- on controller input, convert code to post request - Request.rs
|- On recieving Request, Post code - Posting.rs
*/
