use gilrs::ev::{Button, EventType};
use gilrs::{Event, GamepadId, Gilrs};

pub fn next_event(gamepad_id: GamepadId, gilrs: &mut Gilrs) -> Button {
    loop {
        if let Some(Event {
            id,
            event: EventType::ButtonReleased(button, _),
            time: _,
        }) = gilrs.next_event()
        {
            if id == gamepad_id && button != Button::Unknown {
                return button;
            }
        }
    }
}
