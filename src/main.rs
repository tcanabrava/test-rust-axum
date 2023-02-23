use zero2prod::run;

#[tokio::main]
async fn main() {
    println!("Starting the server");
    let server = run();
    server.await.unwrap();
    println!("Server finished");
}
