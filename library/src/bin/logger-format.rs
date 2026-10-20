use log::error;

fn main() {
    env_logger::init();

    let user = "Yamauchi";
    let id = 42;
    error!("Fatal error: User {user} with ID {id} not found");
}
