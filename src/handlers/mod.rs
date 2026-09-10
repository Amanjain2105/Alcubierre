// --------------------------------------------------
// Handler
// --------------------------------------------------

pub mod account;
mod transaction;

use crate::{AppState, extractor::{JsonRpc, RpcParams}, handlers};
use axum::{Json, extract::{State}};
use serde_json::{Value, json};



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
        let params = payload.params;
        let id = payload.id;
        if let Some(params) = params{
            if let RpcParams::Array(arr) = params{
                if let Some(first) = arr.first(){
                    if let Some(second) = arr.get(1){
                        let address = first.as_str().unwrap_or_default().to_string();
                        let chain_id = second.as_str().unwrap_or_default().to_string();
                        return match handlers::account::get_token_balances(address, chain_id).await{
                            Ok(balances) => Json(json!({
                                "jsonrpc": "2.0",
                                "id": id,
                                "result": balances
                            })),
                            Err(e) => Json(json!({
                                "jsonrpc": "2.0",
                                "id": id,
                                "error": {"code": -32603, "message": e}
                            }))
                        }
                    }
                }
            }
        }
        Json(json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {"code": -32602, "message": "Invalid params"}
        
        }))
    }


    "getSupportedTokens" =>{
        let params = payload.params;
        let id = payload.id;

        if let Some(params) = params{
            if let RpcParams::Array(arr) = params{
                if let Some(first) = arr.first(){
                    let chain_id = first.as_str().unwrap_or_default().to_string();
                    return match handlers::account::get_supported_tokens(chain_id).await {
                        Ok(tokens) => Json(json!({
                            "jsonrpc": "2.0,",
                            "id": id,
                            "result": tokens
                        })),
                        Err(e) => Json(json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "error": {"code": -32603, "message": e }

                        }))
                    }
                }
            }
        }

        Json(json!({
            "jsonrpc": "2.0",
            "result": "",
            "id": id
        
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
        let params = payload.params;
        let id = payload.id;

        if let Some(params) = params{
            if let RpcParams::Array(arr) = params{
                if let (Some(original_hash), Some(raw_bytes)) = (arr.first(), arr.get(1))
                {
                    let original_tx_hash = original_hash.as_str().unwrap_or_default().to_string();

                    let replacement_tx_hash = raw_bytes.as_str().unwrap_or_default().to_string();

                    return handlers::transaction::replace_transaction(
                        State(state), 
                        original_tx_hash,
                        replacement_tx_hash).await

                }
            }
        } 
        Json(json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {"code": -32602, "message": "invalid params"}        
        }))
    }


    "estimateGas"=>{
        let params = payload.params;
        let id = payload.id;

        if let Some(RpcParams::Array(arr)) = params{
            if let Some(from) = arr.first(){
                if let Some(to) = arr.get(1){
                    if let Some(value) = arr.get(2){
                        if let Some(data) = arr.get(3){
                            let from = from.as_str().unwrap_or_default().to_string();
                            let to = to.as_str().unwrap_or_default().to_string();
                            let value = value.as_str().unwrap_or_default().to_string();
                            let data = data.as_str().unwrap_or_default().to_string();

                            return match handlers::transaction::estimate_gas(from, to, value, data).await{
                                Ok(estimate) =>Json(json!({
                                    "jsonrpc": "2.0",
                                    "id": id,
                                    "result": estimate
                                })),
                                Err(e) => Json(json!({
                                    "error": {
                                        "code": -32603,
                                        "message": e
                                    }
                                }))
                            }
                        }
                    }
                }
            }

        } 


        Json(json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": -32602,
                "message": "Invalid Params",
            }
        
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



