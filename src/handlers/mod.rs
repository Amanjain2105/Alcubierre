// --------------------------------------------------
// Handler
// --------------------------------------------------

pub mod account;
mod transaction;

use crate::{AppState, extractor::{JsonRpc, RpcParams}, handlers};
use axum::{Json, extract::State};
use serde_json::{json, Value};



pub async fn rpc(State(state): State<AppState>, JsonRpc(payload): JsonRpc) -> Json<Value> {
   match payload.method.as_str(){
    "getAccountInfo" => {
        let params = payload.params;
        let id = payload.id;

        if let Some(params) = params{
            if let RpcParams::Array(arr) = params{
                if let Some(first) = arr.first(){
                    let address = first.as_str().unwrap_or_default().to_string();
                    return match handlers::account::get_account_info(address).await{
                        Ok(info) => Json(json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": info,
                        })),
                        Err(e) => Json(json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "error": {
                                "code": -32603,
                                "message": e
                            }
                        }))
                    };
                }
            }
        }
        Json(json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": -32602,
                "message": "Invalid params"
            }
        }))

    }
    "getTokenBalances" =>{
        Json(json!({
            "jsonrpc": "2.0",
            "result": "",
            "id": payload.id
        
        }))
    }
    "getSupportedTokens" =>{
        Json(json!({
            "jsonrpc": "2.0",
            "result": "",
            "id": payload.id
        
        }))
    }
    "getTransactionStatus" =>{
        let id = payload.id;
        let params = payload.params;

        if let Some(params) = params{
            if let RpcParams::Array(arr) = params{
                if let Some(first) = arr.first(){
                    if let Some(tx_hash) = first.as_str(){
                        return handlers::transaction::get_transaction_status(State((state)), tx_hash.to_string()).await;
                    }
                }
                
            }
        }
        Json(json!({
            "jsonrpc": "2.0",
            "error": {
                "code": -32602,
                "message": "Invalid params"
            },
            "id": id
        
        }))
    }
    "sendTransaction"=>{
        handlers::transaction::send_transaction(State(state)).await
    }
    "replaceTransaction"=>{
        Json(json!({
            "jsonrpc": "2.0",
            "result": "",
            "id": payload.id
        
        }))
    }
    "estimateGas"=>{
        Json(json!({
            "jsonrpc": "2.0",
            "result": "",
            "id": payload.id
        
        }))
    }
    _ => {
     Json(json!({
         "jsonrpc": "2.0",
         "error": {
             "code": -32601,
             "message": "Method Not Found"
         },
         "id": payload.id
     }))
    }
   }
}



