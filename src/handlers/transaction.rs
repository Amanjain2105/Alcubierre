use std::sync::{Arc, Mutex};

use axum::{Json, extract::State, http::status};
use serde_json::{Value, json};
use uuid::Uuid;
use sqlx::Row;

use crate::{AppState, types::transaction::{GasEstimate, TransactionStatus::{self, Received}}};

pub async fn send_transaction(State(state): State<AppState>) -> Json<Value>{

    let tx_hash = Uuid::new_v4().to_string();

        let result = sqlx::query(
            "INSERT INTO transactions (tx_hash, status) VALUES ($1, $2)"
        )
            .bind(&tx_hash)
            .bind("Received")
            .execute(&state.db)
            .await;

        match result {
            Ok(_)=>{
                Json(json!({
                    "tx_hash": tx_hash
                }))
            }
            Err(e)=>{
                Json(json!({
                    "error": {
                        "code": -32602,
                        "message": "Transaction Not Found"
                    }
                }))
            }
        }
}

pub async fn get_transaction_status(State(state): State<AppState>, tx_hash: String) -> Json<Value> {
    let result = sqlx::query(
        "SELECT status, block_number, gas_used FROM transactions WHERE tx_hash = $1"
    )
    .bind(&tx_hash)
    .fetch_optional(&state.db)
    .await;

    match result {
        Ok(Some(row)) => {
            let status: String = row.get("status");
            let block_number: Option<i64> = row.get("block_number");
            let gas_used: Option<i64> = row.get("gas_used");
            Json(json!({
                "jsonrpc": "2.0",
                "result": {
                    "status": status,
                    "block_number": block_number,
                    "gas_used": gas_used
                }
            }))
        }
        Ok(None) => Json(json!({
            "jsonrpc": "2.0",
            "error": { "code": -32602, "message": "Transaction not found" }
        })),
        Err(e) => Json(json!({
            "jsonrpc": "2.0",
            "error": { "code": -32603, "message": e.to_string() }
        }))
    }
}
pub async fn replace_transaction(
    State(state): State<AppState>,
    original_tx_hash: String,
    _replacement_tx_bytes: String,
) -> Json<Value> {
    let exists = sqlx::query(
        "SELECT tx_hash FROM transactions WHERE tx_hash = $1"
    )
    .bind(&original_tx_hash)
    .fetch_optional(&state.db)
    .await;

    match exists {
        Ok(Some(_)) => {
            let new_hash = Uuid::new_v4().to_string();
            let insert = sqlx::query(
                "INSERT INTO transactions (tx_hash, status) VALUES ($1, $2)"
            )
            .bind(&new_hash)
            .bind("Received")
            .execute(&state.db)
            .await;

            match insert {
                Ok(_) => Json(json!({
                    "jsonrpc": "2.0",
                    "tx_hash": new_hash
                })),
                Err(e) => Json(json!({
                    "jsonrpc": "2.0",
                    "error": { "code": -32603, "message": e.to_string() }
                }))
            }
        }
        Ok(None) => Json(json!({
            "jsonrpc": "2.0",
            "error": { "code": -32602, "message": "Transaction not found" }
        })),
        Err(e) => Json(json!({
            "jsonrpc": "2.0",
            "error": { "code": -32603, "message": e.to_string() }
        }))
    }
}

pub async fn estimate_gas(
    from: String,
    _to: String,
    _value: String,
    _data: String
) -> Result<GasEstimate, String> {
    Ok(GasEstimate { gas_limit: (21000), gas_price: (20000000.to_string())})
}