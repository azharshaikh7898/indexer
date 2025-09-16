 
CREATE TABLE IF NOT EXISTS transactions (
    tx_hash TEXT PRIMARY KEY,
    block_number INTEGER NOT NULL,
    from_address TEXT NOT NULL,
    to_address TEXT NOT NULL,
    amount NUMERIC NOT NULL,
    timestamp INTEGER NOT NULL
);

-- Binance addresses lookup table
CREATE TABLE IF NOT EXISTS binance_addresses (
    address TEXT PRIMARY KEY
);

-- Aggregated net flows
CREATE TABLE IF NOT EXISTS net_flows (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER NOT NULL,
    cumulative_amount NUMERIC NOT NULL,
    UNIQUE(timestamp)
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_transactions_block ON transactions(block_number);
CREATE INDEX IF NOT EXISTS idx_transactions_addresses ON transactions(from_address, to_address);
CREATE INDEX IF NOT EXISTS idx_net_flows_timestamp ON net_flows(timestamp);