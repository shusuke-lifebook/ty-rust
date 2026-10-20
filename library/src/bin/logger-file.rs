use std::fs;

use log::error;

fn main() {
    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("logger-file.log")
        .unwrap();

    env_logger::builder()
        .target(env_logger::Target::Pipe(Box::new(file)))
        .init();

    let user = "Yamauchi";
    let id = 42;
    error!("Fatal error: User {user} with ID {id} not found");
}
