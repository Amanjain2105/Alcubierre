pub mod extractor;
mod types;
mod handlers;
use sqlx::{PgPool, Row, query};
use uuid::Uuid;

use std::{collections::HashMap, sync::{Arc, Mutex}, task::Poll::Pending, time::Duration};

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

    let pool_task1 =pool.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            sqlx::query(
                "UPDATE transactions SET status = $1 WHERE status = $2"
            )
            .bind("Pending")
            .bind("Received")
            .execute(&pool_task1)
            .await
            .ok();

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            sqlx::query(
                "UPDATE transactions SET status = $1, block_number = $2 WHERE status = $3"
            )
            .bind("Included")
            .bind(1_i64)
            .bind("Pending")
            .execute(&pool_task1)
            .await
            .ok();

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            sqlx::query(
                "UPDATE transactions SET status = $1, gas_used = $2 WHERE status = $3"
            )
            .bind("Finalized")
            .bind(21000_i64)
            .bind("Included")
            .execute(&pool_task1)
            .await
            .ok();
        }
    });

    let pool_task2 = pool.clone();
    tokio::spawn(async move{
        let mut block_number: i64 = 1;
        let mut last_block_hash = "genesis".to_string();

        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;

            let block_hash = Uuid::new_v4().to_string();
            let parent_hash = if block_number % 5 == 0{
                "fake parent hash".to_string()
            } else{
                last_block_hash.clone()
            };

            let last_block = sqlx::query(
                "SELECT block_hash FROM blocks ORDER BY block_number DESC LIMIT 1")
            .fetch_optional(&pool_task2)
            .await
            .ok()
            .flatten();

            match last_block{
                Some(row)=>{
                    let db_last_hash: String = row.get("block_hash");
                    if db_last_hash != parent_hash {
                        println!("Reorg detected at block {}", block_number);
                        let affected = sqlx::query(
                            "SELECT tx_hash FROM transactions
                            WHERE block_number >= $1
                            AND status IN ('Included','Finalized')"
                        )
                        .bind(block_number)
                        .fetch_all(&pool_task2)
                        .await
                        .ok()
                        .unwrap_or_default();
                    println!("affected rows: {}", affected.len());
                    
                    sqlx::query("UPDATE transactions
                        SET status = $1, block_number = $2, gas_used = $3, block_hash = $4 WHERE block_number >= $5
                        AND status IN('Included', 'Finalized')")
                    .bind("Pending")
                    .bind(Option::<i64>::None)
                    .bind(Option::<i64>::None)
                    .bind(Option::<String>::None)
                    .bind(block_number)
                    .execute(&pool_task2)
                    .await
                    .ok()
                    .unwrap_or_default();

                for row in &affected{
                        let tx_hash: String = row.get("tx_hash");
                        sqlx::query(
                            "INSERT INTO transaction_events (tx_hash, status)
                            VALUES ($1, $2)"
                        )
                        .bind(&tx_hash)
                        .bind("Pending")
                        .execute(&pool_task2)
                        .await
                        .map_err(|e| eprintln!("rollback update failed: {e}"))
                        .ok();
                        
                    }
                    sqlx::query("DELETE FROM blocks WHERE block_number >= $1")
                        .bind(block_number)
                        .execute(&pool_task2)
                        .await
                    .ok();

                    }

                    
                }
                None =>{
                    

                }
            }

            sqlx::query(
                "INSERT INTO blocks (block_number, block_hash, parent_hash) VALUES ($1, $2, $3)"
            )
            .bind(block_number)
            .bind(&block_hash)
            .bind(&parent_hash)
            .execute(&pool_task2)
            .await
            .ok();
            
            last_block_hash = block_hash;
            block_number += 1;

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



