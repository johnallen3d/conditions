#![deny(clippy::pedantic)]

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();

    match conditions::run().await {
        Ok(result) => println!("{result}"),
        Err(err) => {
            // for user
            eprintln!("{err}");
            // for development
            log::error!("{:?}", err);
        }
    }
}
