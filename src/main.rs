pub mod extractor;
mod types;
mod handlers;

use std::{collections::HashMap, sync::{Arc, Mutex}, task::Poll::Pending};

use axum::{routing::post, Router};
use handlers::rpc;

use crate::types::transaction::TransactionStatus::{self, Received};

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

    let tx_store = Arc::clone(&state.transaction_status);

    tokio::spawn(async move{
        loop{
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            let mut store = tx_store.lock().unwrap(); 
            for (_tx_hash, status) in store.iter_mut(){
                 match status {
                    TransactionStatus::Received => {
                        *status = TransactionStatus::Pending { queue_position: (1) }
                    }
                    TransactionStatus::Pending { queue_position} =>{
                        *status = TransactionStatus::Included { block_number: (1) }
                    }
                    TransactionStatus::Included { block_number }=>{
                        *status = TransactionStatus::Finalized { block_number: 1, gas_used: 34 }
                    }
                    TransactionStatus::Finalized {..}=>{}

                    TransactionStatus::Failed {..}=>{}
                 }
            }
        }
    });

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



