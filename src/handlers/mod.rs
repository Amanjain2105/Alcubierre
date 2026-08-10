// --------------------------------------------------
// Handler
// --------------------------------------------------

pub mod account;

use crate::{extractor::{JsonRpc, RpcParams}, handlers};
use axum::Json;
use serde_json::{json, Value};



pub async fn rpc(JsonRpc(payload): JsonRpc) -> Json<Value> {
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
        Json(json!({
            "jsonrpc": "2.0",
            "result": "",
            "id": payload.id
        
        }))
    }
    "sendTransaction"=>{
        Json(json!({
            "jsonrpc": "2.0",
            "result": "",
            "id": payload.id
        }))
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



