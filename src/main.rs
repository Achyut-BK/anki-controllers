use log::{info, LevelFilter};
mod controller;
mod init;
mod post;

fn main() {
    match init::init_log(Some(LevelFilter::Info)) {
        Ok(_) => info!("Log Initializaton successful, continuing"),
        Err(err) => panic!("Log Initializaton failed with {:?}", err),
    };

    let (active_gamepad_id, mut gilrs) = match init::init_gamepad() {
        Ok((gamepad, gilrs)) => {
            info!("Gamepad Initializaton successful, continuing");
            (gamepad, gilrs)
        }
        Err(err) => panic!("Gamepad Initializaton failed with {:?}", err),
    };

    let client = match init::init_anki_client() {
        Ok(client) => {
            info!("HTTPS Client Initializaton successful, continuing");
            client
        }
        Err(err) => panic!("HTTPS Client Initializaton failed with {:?}", err),
    };

    let headers = post::get_headers();

    match init::init_anki(client, headers) {
        Ok(_) => info!("Anki connection successful, continuing"),
        Err(err) => panic!("Anki connection failed with {:?}", err),
    };

    loop {
        println!(
            "{:?}",
            controller::next_event(active_gamepad_id, &mut gilrs)
        );
        // check for controller input
        // When recieve controller input create request
        // When recieve request, post
    }
}

/*
Main
|
|- Wait for anki and Controllers to be connected - Init.rs
|- wait for controller input - Controller.rs
|- on controller input, convert code to post request - Request.rs
|- On recieving Request, Post code - Post.rs
*/
