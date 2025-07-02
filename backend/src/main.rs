// #![deny(missing_docs)]
use backend::start;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start().await
}
