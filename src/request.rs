use gilrs::ev::Button;
use serde_json::{json, Value};

pub fn generate(button: Button) -> Value {
    match button {
        Button::Mode => json!({
            "version": 6,
            "action": "guiExitAnki",
        }),
        _ => {
            println!("{:?}", button);
            json!({})
        }
    }
}
