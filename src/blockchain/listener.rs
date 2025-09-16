use web3::types::{BlockNumber, FilterBuilder, Log, H160, U64};
use web3::Web3;
use std::str::FromStr;

pub struct BlockchainListener {
    web3: Web3<web3::transports::Http>,
    pol_contract: H160,
}

impl BlockchainListener {
    pub fn new(rpc_url: &str, pol_contract: &str) -> Result<Self, web3::Error> {
        let transport = web3::transports::Http::new(rpc_url)?;
        let web3 = Web3::new(transport);
        let pol_contract = H160::from_str(pol_contract)
            .map_err(|_| web3::Error::InvalidResponse("Invalid contract address".into()))?;

        Ok(Self { web3, pol_contract })
    }

    pub async fn listen_transfers(&self, from_block: U64) -> Result<Vec<Log>, web3::Error> {
        let filter = FilterBuilder::default()
            .from_block(BlockNumber::Number(from_block))
            .address(vec![self.pol_contract])
            .topics(
                Some(vec![web3::types::H256::from_str(
                    "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
                ).unwrap()]),
                None,
                None,
                None,
            )
            .build();

        self.web3.eth().logs(filter).await
    }

    pub async fn get_block_timestamp(&self, block_number: U64) -> Result<U64, web3::Error> {
        let block = self.web3.eth()
            .block(web3::types::BlockId::Number(BlockNumber::Number(block_number)))
            .await?
            .ok_or(web3::Error::InvalidResponse("Block not found".into()))?;
        
        Ok(U64::from(block.timestamp.low_u64()))
    }
}