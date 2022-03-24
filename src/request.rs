use crate::post;
use gilrs::ev::Button;
use reqwest::blocking::Client;
use serde_json::{json, Value};

pub fn generate(button: Button, client: &Client) -> Value {
    let mut current_card_id: u64 = 0;
    if let Ok(card) = post::post(
        client,
        json!({
            "version": 6,
            "action": "guiCurrentCard"
        }),
    ) {
        current_card_id = card["result"]["cardId"].as_u64().unwrap();
    };
    match button {
        Button::Mode => json!({
            "version": 6,
            "action": "guiExitAnki",
        }),
        Button::Select => json!({
            "version": 6,
            "action": "guiDeckBrowser",
        }),
        Button::Start => json!({
            "version": 6,
            "action": "sync",
        }),
        Button::RightTrigger2 => json!({
            "version": 6,
            "action": "guiShowAnswer",
        }),
        Button::RightTrigger => json!({
            "version": 6,
            "action": "guiAddCards",
        }),
        Button::North => json!({
            "action": "multi",
            "version": 6,
            "params": {
                "actions": [
                    {
                        "action": "deleteNotes",
                        "params": {
                            "notes": [current_card_id]
                        }
                    },
                    {
                        "action": "guiCheckDatabase",
                    },

                ]
            }
        }),
        Button::East => json!({
            "action": "multi",
            "version": 6,
            "params": {
                "actions": [
                    {
                        "action": "suspend",
                        "params": {
                            "cards": [current_card_id]
                        }
                    },
                    {
                        "action": "guiCheckDatabase",
                    },
                ]
            }
        }),
        Button::South => json!({
            "version": 6,
            "action": "guiStartCardTimer",
        }),
        Button::West => json!({
            "action": "multi",
            "version": 6,
            "params": {
                "actions": [
                    {
                        "action": "forgetCards",
                        "params": {
                            "cards": [current_card_id]
                        }
                    },
                    {
                        "action": "guiCheckDatabase",
                    },
                ]
            }
        }),
        Button::DPadDown => generate_dpad_result(1),
        Button::DPadLeft => generate_dpad_result(2),
        Button::DPadUp => generate_dpad_result(3),
        Button::DPadRight => generate_dpad_result(4),
        Button::LeftTrigger => json!({
            "version": 6,
            "action": "guiBrowse",
        }),
        Button::LeftTrigger2 => json!({
            "version": 6,
            "action": "guiShowQuestion",
        }),
        _ => {
            println!("{:?} is not supported", button);
            json!({})
        }
    }
}

fn generate_dpad_result(ease: u32) -> Value {
    json!({
        "version": 6,
        "action": "guiAnswerCard",
        "params": {
            "ease": ease
        }
    })
}
