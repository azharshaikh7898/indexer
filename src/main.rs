mod database;
mod blockchain;
mod processor;
mod api;

use std::path::PathBuf;
use clap::Parser;
use tokio::time::{sleep, Duration};
use web3::types::U64;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    rpc_url: String,

    #[arg(short = 'c', long)]
    pol_contract: String,

    #[arg(short, long)]
    db_path: PathBuf,

    #[arg(short = 'p', long, default_value = "3000")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize database
    let db = database::Database::new(&args.db_path)?;
    db.init()?;

    // Initialize blockchain listener
    let listener = blockchain::listener::BlockchainListener::new(&args.rpc_url, &args.pol_contract)?;

    // Initialize processor
    let processor = processor::Processor::new(db, listener);

    // Initialize API (pass db_path as String, not Database)
    let api = api::Api::new(args.db_path.to_string_lossy().to_string());
    let app = api.router();

    // Start the API server
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], args.port));
    println!("Starting server on {}", addr);

    // Run the API server and blockchain processor concurrently
    tokio::select! {
        _ = axum::Server::bind(&addr).serve(app.into_make_service()) => {},
        _ = async {
            let mut current_block = U64::from(0);
            loop {
                if let Err(e) = processor.process_block(current_block).await {
                    eprintln!("Error processing block {}: {}", current_block, e);
                }
                // Correct U64 arithmetic:
                current_block = current_block + U64::one();
                sleep(Duration::from_secs(1)).await;
            }
        } => {}
    }

    Ok(())
}