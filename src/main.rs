use base64::{prelude::BASE64_STANDARD as b64, Engine};
use clap::Parser;
use solana_sdk::transaction::{Transaction, VersionedTransaction};
use soltx::{App, TransactionVariant};

#[derive(Debug, Parser)]
struct Args {
    /// Provide the Transaction
    tx: String,

    /// Use RPC mode which parses balance & tx count (requires the env var 'RPC' to be set)
    #[clap(short, long, default_value_t = false)]
    rpc: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    let args = Args::parse();

    let app = App::new(args.rpc);

    let tx_bytes = b64.decode(args.tx).expect("valid tx");
    let tx = {
        if let Ok(tx) = bincode::deserialize::<Transaction>(&tx_bytes) {
            TransactionVariant::Transaction(tx)
        } else {
            TransactionVariant::VersionedTransaction(
                bincode::deserialize::<VersionedTransaction>(&tx_bytes)
                    .expect("Require Transaction or Versioned Transaction"),
            )
        }
    };
    let table = app.build_account_table(&tx).await?;
    println!("{}", table);

    let table = app.build_instruction_table(&tx).await?;
    println!("{}", table);

    Ok(())
}
