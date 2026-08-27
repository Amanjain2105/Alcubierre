pub mod extractor;
mod types;
mod handlers;

use std::{collections::HashMap, sync::{Arc, Mutex}};

use axum::{routing::post, Router};
use handlers::rpc;

use crate::types::transaction::TransactionStatus;

#[derive(Clone)]
struct AppState{
    transaction_status: Arc<Mutex<HashMap<String, TransactionStatus>>>
}



// --------------------------------------------------
// Main
// --------------------------------------------------

#[tokio::main]
async fn main() {

    let state = AppState{
        transaction_status: Arc::new(Mutex::new(HashMap::new())),
    };



    let app = 
    Router::new().route("/rpc", post(rpc)).
    with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}


/*getAccountInfo
getTokenBalances
getSupportedTokens
estimateGas
sendTransaction
getTransactionStatus
replaceTransaction */