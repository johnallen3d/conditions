fn main() {
    env_logger::init();

    match conditions::run() {
        Ok(_) => (),
        Err(err) => {
            // for user
            eprintln!("{}", err);
            // for development
            log::error!("{:?}", err);
        }
    }
}
