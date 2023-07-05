fn main() {
    env_logger::init();

    match conditions::run() {
        Ok(_) => (),
        Err(err) => log::error!("{}", err),
    }
}
