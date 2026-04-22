use clap::Parser;
use zeroback::{Args, run};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    run(args).await
}
