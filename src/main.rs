#![deny(clippy::pedantic)]

fn main() {
    env_logger::init();

    match conditions::run() {
        Ok(result) => println!("{result}"),
        Err(err) => {
            // for user
            eprintln!("{err}");
            // for development
            log::error!("{:?}", err);
        }
    }
}
