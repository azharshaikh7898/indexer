# Real-time Polygon Blockchain Data Indexer

## 📌 Overview
This project is a **real-time blockchain data indexer** for the Polygon (POL) network.  
It listens to the Polygon blockchain, extracts transaction data, and calculates **cumulative net flows** of POL tokens to the Binance exchange (or other supported exchanges in the future).  

The system stores parsed blockchain data in a database (SQLite by default) and provides an **HTTP API** (via [Axum](https://github.com/tokio-rs/axum)) for querying indexed information.

---

## ✨ Features
- Real-time ingestion of Polygon blockchain transactions
- Tracks token transfers and exchange inflows/outflows
- Cumulative **net-flow calculation** to Binance
- Exposes a REST API endpoint (`/net-flow`) to query results
- Modular design for adding new exchanges in the future
- Lightweight database support (SQLite)

---

## 🛠️ Tech Stack
- **Rust** (main language)
- **Axum** (web framework)
- **Tokio** (async runtime)
- **SQLite** (storage backend)
- **Web3 / RPC** (Polygon blockchain data source)

---

## 🚀 Getting Started

### 1️⃣ Clone the repository
```bash
git clone https://github.com/azharshaikh7898/Polygon-Blockchain-indexer.git
cd Polygon-Blockchain-indexer
2️⃣ Install dependencies
Make sure you have Rust installed:

bash
Copy code
rustup install stable
rustup default stable
3️⃣ Configure environment
Create a .env file with your Polygon RPC provider URL:

env
Copy code
POLYGON_RPC_URL=https://polygon-rpc.com
DATABASE_URL=netflow.db
4️⃣ Run the indexer
bash
Copy code
cargo run
