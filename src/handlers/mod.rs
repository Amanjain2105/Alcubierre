// --------------------------------------------------
// Handler
// --------------------------------------------------

use crate::extractor::JsonRpc;
use axum::{
    extract::{
        rejection::JsonRejection,
        FromRequest,
        Request,
    },
    http::StatusCode,
    routing::post,
    Json, Router,
};
use serde_json::{json, Map, Value};
pub async fn rpc(JsonRpc(payload): JsonRpc) -> Json<Value> {
   match payload.method.as_str(){
    "getAccountInfo" => {
        Json(json!({
            "jsonrpc": "2.0",
            "result": "",
            "id": payload.id
        
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