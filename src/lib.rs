use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::CompiledInstruction,
    pubkey::Pubkey,
    transaction::{Transaction, VersionedTransaction},
};
use tabled::{
    builder,
    settings::{object::Columns, Alignment, Modify, Style, Width},
    Table,
};

pub struct App {
    rpc: Option<RpcClient>,
}

pub trait Tx {
    fn get_account_keys(&self) -> &Vec<Pubkey>;
    fn get_instructions(&self) -> &Vec<CompiledInstruction>;
}

pub enum TransactionVariant {
    Transaction(Transaction),
    VersionedTransaction(VersionedTransaction),
}

impl TransactionVariant {
    fn get_account_keys(&self) -> &Vec<Pubkey> {
        match self {
            TransactionVariant::Transaction(tx) => &tx.message.account_keys,
            TransactionVariant::VersionedTransaction(tx) => match &tx.message {
                solana_sdk::message::VersionedMessage::Legacy(msg) => &msg.account_keys,
                solana_sdk::message::VersionedMessage::V0(msg) => &msg.account_keys,
            },
        }
    }

    fn get_instructions(&self) -> &Vec<CompiledInstruction> {
        match self {
            TransactionVariant::Transaction(tx) => &tx.message.instructions,
            TransactionVariant::VersionedTransaction(tx) => match &tx.message {
                solana_sdk::message::VersionedMessage::Legacy(msg) => &msg.instructions,
                solana_sdk::message::VersionedMessage::V0(msg) => &msg.instructions,
            },
        }
    }
}

pub fn get_tx_count(n: usize) -> String {
    {
        if n == 25 {
            "25+".to_string()
        } else {
            n.to_string()
        }
    }
}

impl App {
    pub fn new(rpc: bool) -> Self {
        if rpc {
            let rpc = std::env::var("RPC").expect("Missing env var 'RPC'");
            let rpc = Some(RpcClient::new(rpc));
            Self { rpc }
        } else {
            Self { rpc: None }
        }
    }

    pub async fn build_account_table(&self, tx: &TransactionVariant) -> anyhow::Result<Table> {
        let mut builder = builder::Builder::new();
        // build the header
        if self.rpc.is_some() {
            builder.push_record([
                "Account".to_string(),
                "Balance".to_string(),
                "# TX".to_string(),
            ]);
        } else {
            builder.push_record(["Account".to_string()]);
        }

        for account in tx.get_account_keys() {
            if let Some(ref rpc) = self.rpc {
                let balance = rpc.get_balance(account).await?;
                let signatures = rpc
                    .get_signatures_for_address_with_config(
                        account,
                        solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config {
                            limit: Some(25),
                            ..Default::default()
                        },
                    )
                    .await?;
                builder.push_record([
                    account.to_string(),
                    balance.to_string(),
                    get_tx_count(signatures.len()),
                ]);
            } else {
                builder.push_record([account.to_string()]);
            }
        }

        let mut table = builder.build();
        table.with(Style::sharp());

        Ok(table)
    }

    pub async fn build_instruction_table(&self, tx: &TransactionVariant) -> anyhow::Result<Table> {
        let mut builder = builder::Builder::new();
        // build the header
        builder.push_record(["Program ID".to_string(), "Data".to_string()]);

        for instruction in tx.get_instructions() {
            builder.push_record([
                tx.get_account_keys()[instruction.program_id_index as usize].to_string(),
                instruction
                    .data
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ]);
        }

        let mut table = builder.build();
        table.with(Style::sharp()).with(
            Modify::new(Columns::single(1))
                .with(Width::wrap(50))
                .with(Alignment::right()),
        );

        Ok(table)
    }
}
