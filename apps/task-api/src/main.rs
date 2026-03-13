#[tokio::main]
async fn main() {
    dotenv::from_filename(".env.local").ok();
    api::run().await;
}
