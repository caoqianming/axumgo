mod server;
mod apps;


#[tokio::main]
async fn main() {
    server::start::start().await;
}
