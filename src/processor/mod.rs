use crate::database::Database;
use crate::blockchain::listener::BlockchainListener;
use web3::types::{Log, U64, U256};

pub struct Processor {
    db: Database,
    listener: BlockchainListener,
}

impl Processor {
    pub fn new(db: Database, listener: BlockchainListener) -> Self {
        Self { db, listener }
    }

    pub async fn process_block(&self, block_number: U64) -> Result<(), Box<dyn std::error::Error>> {
        let logs = self.listener.listen_transfers(block_number).await?;
        let timestamp = self.listener.get_block_timestamp(block_number).await?;

        for log in logs {
            self.process_transfer_log(log, timestamp).await?
        }

        Ok(())
    }

    async fn process_transfer_log(&self, log: Log, timestamp: U64) -> Result<(), Box<dyn std::error::Error>> {
        let topics = log.topics;
        if topics.len() != 3 {
            return Ok(());  // Not a transfer event
        }

        let from = format!("0x{}", &topics[1].to_string()[26..]);
        let to = format!("0x{}", &topics[2].to_string()[26..]);
        let amount = U256::from_big_endian(&log.data.0);

        self.db.insert_transaction(
            &log.transaction_hash.unwrap().to_string(),
            log.block_number.unwrap().as_u64() as i64,
            &from,
            &to,
            &amount.to_string(),
            timestamp.as_u64() as i64,
        )?;

        Ok(())
    }
}