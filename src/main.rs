use reqwest::get;
use zero2prod::startup::run;
use zero2prod::configuration::*;

#[tokio::main]
async fn main() {
    println!("Starting the server");

    let config = match get_config() {
        Ok(config) => config,
        Err(err) => panic!("{}", err)
    };

    let address = format!("127.0.0.1:{}", config.application_port);

    let listener = std::net::TcpListener::bind(&address)
        .expect("Could not bind to address");

    let server = run(listener);
    server.await.unwrap();
    println!("Server finished");
}
