use log::{info, LevelFilter};
mod controller;
mod init;
mod post;
mod request;

fn main() {
    match init::init_log(Some(LevelFilter::Info)) {
        Ok(_) => info!("Log Initializaton successful, continuing"),
        Err(err) => panic!("Log Initializaton failed with {:?}", err),
    };

    println!("Press the Start or \"Review Cards\" button to start");

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

    match init::init_anki(&client) {
        Ok(_) => info!("Anki connection successful, continuing"),
        Err(err) => panic!("Anki connection failed with {:?}", err),
    };

    loop {
        println!(
            "{:?}",
            post::post(
                &client,
                request::generate(
                    controller::next_event(active_gamepad_id, &mut gilrs),
                    &client,
                )
            )
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
