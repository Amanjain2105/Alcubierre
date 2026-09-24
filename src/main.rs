pub mod extractor;
mod types;
mod handlers;
use sqlx::PgPool;

use std::{collections::HashMap, sync::{Arc, Mutex}, task::Poll::Pending};

use axum::{routing::post, Router};
use handlers::rpc;

use crate::types::transaction::TransactionStatus::{self, Received};

#[derive(Clone)]
struct AppState{
    db:PgPool,
}



// --------------------------------------------------
// Main
// --------------------------------------------------

#[tokio::main]
async fn main() {
    let database_url = "postgres://amanjain@localhost/alcubierre";
    
    let pool = PgPool::connect(database_url)
        .await
        .expect("Failed to connect to database");

    let state = AppState { db: pool.clone() };

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            sqlx::query(
                "UPDATE transactions SET status = $1 WHERE status = $2"
            )
            .bind("Pending")
            .bind("Received")
            .execute(&pool)
            .await
            .ok();

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            sqlx::query(
                "UPDATE transactions SET status = $1, block_number = $2 WHERE status = $3"
            )
            .bind("Included")
            .bind(1_i64)
            .bind("Pending")
            .execute(&pool)
            .await
            .ok();

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            sqlx::query(
                "UPDATE transactions SET status = $1, gas_used = $2 WHERE status = $3"
            )
            .bind("Finalized")
            .bind(21000_i64)
            .bind("Included")
            .execute(&pool)
            .await
            .ok();
        }
    });

    let app = Router::new()
        .route("/rpc", post(rpc))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}



