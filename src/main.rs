use zero2prod::startup::run;

#[tokio::main]
async fn main() {
    println!("Starting the server");
    let listener = std::net::TcpListener::bind("127.0.0.1:8000")
        .expect("Could not bind to address");
    let server = run(listener);
    server.await.unwrap();
    println!("Server finished");
}
