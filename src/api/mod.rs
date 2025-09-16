use axum::{
    routing::get,
    Router,
    Json,
    extract::State,
};
use std::path::Path;
use serde_json::json;

// Correctly import the Database module
use crate::database::Database;

// If you need to use the listener elsewhere in this file, import it like this:
// use crate::blockchain::listener::BlockchainListener;

#[derive(Clone)]
pub struct Api {
    db_path: String,
}

impl Api {
    pub fn new(db_path: String) -> Self {
        Self { db_path }
    }

    pub fn router(&self) -> Router {
        Router::new()
            .route("/net-flow", get(Self::get_net_flow))
            .with_state(self.clone())
    }

    async fn get_net_flow(State(api): State<Api>) -> Json<serde_json::Value> {
        let db = Database::new(Path::new(&api.db_path)).unwrap();
        match db.get_net_flow() {
            Ok(net_flow) => Json(json!({ "net_flow": net_flow })),
            Err(e) => Json(json!({ "error": e.to_string() })),
        }
    }
}