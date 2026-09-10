use std::sync::{Arc, Mutex};

use axum::{Json, extract::State, http::status};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::{AppState, types::transaction::{GasEstimate, TransactionStatus::{self, Received}}};

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
pub async fn replace_transaction(
    State(state): State<AppState>,
    original_tx_hash: String,
    replacement_tx_hash: String) -> Json<Value> {
        let mut app_state = state.transaction_status.lock().unwrap();
        let new_hash = Uuid::new_v4().to_string();

        match app_state.get(&original_tx_hash){
            Some ( _ ) => {
               let _= app_state.insert(new_hash.clone(), TransactionStatus::Received);
            }
            None => {
                return Json(json!({
                    "error": "Transaction has Failed"
                }))

            }
        }
        
        Json(json!({
            "tx_hash": new_hash
        }))

}

pub async fn estimate_gas(
    from: String,
    _to: String,
    _value: String,
    _data: String
) -> Result<GasEstimate, String> {
    Ok(GasEstimate { gas_limit: (21000), gas_price: (20000000.to_string())})
}