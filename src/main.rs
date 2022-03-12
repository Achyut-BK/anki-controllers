use log::LevelFilter;
mod init;

fn main() {
    //  Initialize logger
    init::init_log(Some(LevelFilter::Info));
    init::init_gamepad();
}

/*
Main
|
|- Wait for anki and Controllers to be connected - Init.rs
|- wait for controller input - Controller.rs
|- on controller input, convert code to post request - Request.rs
|- On recieving Request, Post code - Posting.rs
*/
