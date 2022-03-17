use crate::post;
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use serde_json::json;

pub fn init_anki(client: Client, headers: HeaderMap) -> Result<(), post::Error> {
    let result = post::post(
        client,
        headers,
        json!({
            "version": 6,
            "action": "requestPermission"
        }),
    )?;
    match &result["result"]["permission"] {
        serde_json::Value::String(is_granted) if is_granted == "granted" => Ok(()),
        _ => Err(post::Error::AnkiError {
            err: "Permission to access anki not granted".to_string(),
        }),
    }
}

pub fn init_anki_client() -> Result<Client, reqwest::Error> {
    Client::builder()
        .no_gzip()
        .no_brotli()
        .no_deflate()
        .pool_idle_timeout(None)
        .build()
}

use gilrs::ev::{Button, EventType};
use gilrs::{Event, GamepadId, Gilrs, MappingSource};
use log::{info, warn};

pub fn init_gamepad() -> Result<(GamepadId, Gilrs), gilrs::Error> {
    let mut gilrs = Gilrs::new()?;
    let active_gamepad_id;

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
            active_gamepad_id = id;
            break;
        }
    }
    Ok((active_gamepad_id, gilrs))
}

use log::{LevelFilter, SetLoggerError};

pub fn init_log(log_level: Option<LevelFilter>) -> Result<(), SetLoggerError> {
    let mut builder = env_logger::Builder::new();

    if let Some(level) = log_level {
        builder.filter_level(level);
    };
    builder.try_init()
}
