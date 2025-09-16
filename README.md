# Real-time Polygon Blockchain Data Indexer

A Rust-based indexer that listens to the Polygon (POL) blockchain in real time to track token flows and compute **net cumulative flow** of POL tokens to (and from) Binance (with room to support more exchanges). It stores data locally (SQLite by default) and serves a REST API for querying net flow statistics.

---

## Table of Contents

1. [Features](#features)  
2. [Tech Stack](#tech-stack)  
3. [Architecture & Components](#architecture--components)  
4. [Getting Started](#getting-started)  
   - Prerequisites  
   - Setup & Configuration  
   - Running the Indexer  
   - Using the REST API  
5. [Project Structure](#project-structure)  
6. [Configuration Options](#configuration-options)  
7. [Extending / Customization](#extending--customization)  
8. [Error Handling & Logging](#error-handling--logging)  
9. [Limitations & Todo](#limitations--todo)  
10. [License](#license)  
11. [Contact / Contributions](#contact--contributions)

---

## Features

- Real-time ingestion of Polygon blockchain transactions  
- Extraction of token transfer events  
- Tracking of inflows and outflows of POL tokens to specified exchanges (currently Binance)  
- Calculation of **cumulative net flow** for an exchange (in minus out) over time  
- REST API endpoint (`/net-flow`) to query net flow values  
- Lightweight storage via SQLite (can be swapped out)  
- Modular design to make it easier to add more exchanges in future  

---

## Tech Stack

| Component | Technology / Library |
|-----------|----------------------|
| Language | Rust |
| Async & concurrency | Tokio |
| Web framework | Axum |
| Blockchain RPC / Web3 client | (whatever you are using for Polygon RPC) |
| Database / Storage | SQLite (default) |
| API | REST (HTTP) |
| Configuration / Environment | `.env` or environment variables |

---

## Architecture & Components

Here is how the system is structured:

- **Blockchain Listener** — continuously polls (or subscribes to) Polygon via RPC, extracts relevant transactions (token transfers).  
- **Parser** — filters and parses events to focus on transfers involving tracked exchanges.  
- **Storage** — persists raw / processed events and aggregated metrics (net flow) into a database.  
- **API Layer** — exposes HTTP endpoints to query net flow and maybe additional statistics (e.g. by time window, by exchange).  
- **Scheduler / Aggregator** — component that updates the net flow (incremental, cumulative) as new events come in.

---

## Getting Started

### Prerequisites

- Rust toolchain (stable) — see [rustup.rs](https://rustup.rs/)  
- Cargo (comes with Rust)  
- Access to a Polygon RPC node / provider (e.g. Infura, Alchemy, or a public RPC)  
- SQLite (if using the default DB)  

### Setup & Configuration

1. Clone the repo:

   ```bash
   git clone https://github.com/azharshaikh7898/indexer.git
   cd indexer


Install Rust (if not already) and ensure you have a working toolchain:

rustup install stable
rustup default stable


3. Configure environment variables. Create a .env file in the root directory (or set environment variables directly). Example .env:

POLYGON_RPC_URL=https://polygon-rpc.com
DATABASE_URL=netflow.db
# Optional: more settings


4. Build and run the application:

cargo build --release
cargo run --release

Alternatively for development:

cargo run



Using the REST API

Once running, the HTTP server will start on a configured port (if set; otherwise a default).

Example endpoint:

GET /net-flow

Returns the current cumulative net flow to Binance. You may want endpoints like:

/net-flow?exchange=Binance

/net-flow?exchange=Binance&from=2025-09-01&to=2025-09-16

/net-flow?exchange=Binance&token=POL


Response format should be JSON, e.g.:

{
  "exchange": "Binance",
  "net_flow": 12345.678,
  "period": {
    "start": "2025-09-01T00:00:00Z",
    "end": "2025-09-16T00:00:00Z"
  }
}
