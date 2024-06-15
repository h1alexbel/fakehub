use server::Server;

#[tokio::main]
async fn main() {
    let server = Server::new(3000);
    server.start().await.expect("Can't start the server");
}
