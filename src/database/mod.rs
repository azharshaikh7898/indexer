use sqlite::{Connection, State};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &Path) -> Result<Self, sqlite::Error> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }

    pub fn init(&self) -> Result<(), sqlite::Error> {
        let schema = include_str!("../schema.sql");
        self.conn.execute(schema)?;
        Ok(())
    }

    pub fn insert_transaction(
        &self,
        tx_hash: &str,
        block_number: i64,
        from_address: &str,
        to_address: &str,
        amount: &str,
        timestamp: i64,
    ) -> Result<(), sqlite::Error> {
        let query = "INSERT INTO transactions (tx_hash, block_number, from_address, to_address, amount, timestamp) \
                     VALUES (?, ?, ?, ?, ?, ?)";
        let mut statement = self.conn.prepare(query)?;
        statement.bind((1, tx_hash))?;
        statement.bind((2, block_number))?;
        statement.bind((3, from_address))?;
        statement.bind((4, to_address))?;
        statement.bind((5, amount))?;
        statement.bind((6, timestamp))?;
        match statement.next()? {
            State::Done => Ok(()),
            _ => Ok(()), // treat unexpected results as success/no-op
        }
    }

    pub fn get_net_flow(&self) -> Result<String, sqlite::Error> {
        let query = "SELECT COALESCE(SUM(CASE \
                        WHEN to_address IN (SELECT address FROM binance_addresses) THEN amount \
                        WHEN from_address IN (SELECT address FROM binance_addresses) THEN -amount \
                        ELSE 0 END), '0') as net_flow \
                     FROM transactions";
        
                let mut statement = self.conn.prepare(query)?;
                if let State::Row = statement.next()? {
                    Ok(statement.read::<String, _>(0)?)
                } else {
                    Ok("0".to_string())
                }
            }
        }