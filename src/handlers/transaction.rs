use std::sync::{Arc, Mutex};

use axum::{Json, extract::State, http::status};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::{AppState, types::transaction::TransactionStatus::{self, Received}};

pub async fn send_transaction(State(state): State<AppState>) -> Json<Value>{

    let tx_hash = Uuid::new_v4().to_string();

    {
        let mut app_state = state.transaction_status.lock().unwrap();
        let _= app_state.insert(tx_hash.clone(), TransactionStatus::Received);

    }

    Json(json!({
        "tx_hash": tx_hash
    }))
}

pub async fn get_transaction_status(State(state): State<AppState>, tx_hash: String) -> Json<Value>{
    
        let app_state = state.transaction_status.lock().unwrap();

        match app_state.get(&tx_hash) {
            Some(status)=>{
                Json(json!({
                    "result": status
                }))
            }
            None=>{
                Json(json!({
                    "error": {
                        "code": -32602,
                        "message": "Transaction Not Found"
                    }
                }))
            }
        }


}