pub mod extractor;
mod types;
mod handlers;
use sqlx::{PgPool, Row, query};
use uuid::Uuid;

use std::{collections::HashMap, sync::{Arc, Mutex}, task::Poll::Pending, time::Duration};

use axum::{routing::post, Router};
use handlers::rpc;

use crate::types::transaction::TransactionStatus::{self, Received};

const REORG_DEPTH: i64 = 2;
const FINALITY_DEPTH: i64 = 2;

#[derive(Clone)]
struct AppState{
    db:PgPool,
}

async fn ingest_block(
    pool: &PgPool,
    block_number: i64,
    last_block_hash: &str,
    is_reorg: bool,
) -> Result<(i64, String), sqlx::Error>{
    let fork = (block_number - REORG_DEPTH).max(1);
    let new_hash = Uuid::new_v4().to_string();
    let new_number = if is_reorg{fork} else{block_number};
    let mut tx  = pool.begin().await?;

    if is_reorg{
        println!("Reorg detected at block {}", block_number);

        let affected = sqlx::query(
             "SELECT tx_hash FROM transactions
                WHERE block_number >= $1
                AND status IN ('Included','Finalized')"
            )
            .bind(fork)
            .fetch_all(&mut *tx)
            .await?;
        println!("affected rows: {}", affected.len());
                    
      sqlx::query("UPDATE transactions
          SET status = $1, block_number = $2, gas_used = $3, block_hash = $4 WHERE block_number >= $5
          AND status IN('Included', 'Finalized')"
        )
        .bind("Pending")
        .bind(Option::<i64>::None)
        .bind(Option::<i64>::None)
        .bind(Option::<String>::None)
        .bind(fork)
        .execute(&mut *tx)
        .await?;
                    

        for row in &affected{
            let tx_hash: String = row.get("tx_hash");
                    sqlx::query(
                        "INSERT INTO transaction_events (tx_hash, status)
                        VALUES ($1, $2)"
                    )
                    .bind(&tx_hash)
                    .bind("Pending")
                    .execute(&mut *tx)
                    .await?;
                }
                    sqlx::query("DELETE FROM blocks WHERE block_number >= $1")
                        .bind(fork)
                        .execute(&mut *tx)
                        .await?;
                    }
    let parent_hash: String = if is_reorg{
        let row = sqlx::query("SELECT block_hash FROM blocks ORDER BY block_number DESC LIMIT 1")
            .fetch_optional(&mut *tx)
            .await?;
        match row {
            Some(r) => r.get("block_hash"),
            None => "genesis".to_string(),
        }

    }else{
        last_block_hash.to_string()
    };


    let last_block = sqlx::query(
        "SELECT block_hash FROM blocks ORDER BY block_number DESC LIMIT 1")
        .fetch_optional(&mut *tx)
        .await?;
    sqlx::query(
        "INSERT INTO blocks (block_number, block_hash, parent_hash) VALUES ($1, $2, $3)"
        )
        .bind(new_number)
        .bind(&new_hash)
        .bind(&parent_hash)
        .execute(&mut *tx)
        .await?; 

    sqlx::query(
        "UPDATE transactions SET status = 'Included',
         block_number = $1, block_hash = $2 WHERE status = 'Pending'"
        )
    .bind(new_number)
    .bind(&new_hash)
    .execute(&mut *tx)
    .await?;

    sqlx::query("UPDATE transactions SET status = 'Finalized',
    gas_used = 21000 WHERE status = 'Included' AND block_number <= $1")
    .bind(new_number - FINALITY_DEPTH)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;


    Ok((new_number + 1, new_hash))
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

           if let Err(e) = sqlx::query("UPDATE transactions SET status = $1 WHERE status =$2")
            .bind("Pending")
            .bind("Received")
            .execute(&pool_task1)
            .await{
                eprintln!("recieved -> Pending falied {e}")
            }
           
        }
    });

    let pool_task2 = pool.clone();
    tokio::spawn(async move{

        let last = sqlx::query(
            "SELECT block_number, block_hash FROM blocks
            ORDER BY block_number DESC LIMIT 1"
        )
        .fetch_optional(&pool_task2)
        .await
        .map_err(|e| eprintln!("startup block lookup failed: {e}"))
        .ok()
        .flatten();
        
        let (mut block_number, mut last_block_hash) = match last{
            Some(row) =>{
                let n: i64 = row.get("block_number");
                let h: String = row.get("block_hash");
                (n + 1, h)

            }
            None => (1, "genesis".to_string()),
        };

        let mut tick:u64 = 0;


        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;

            tick += 1;
            
            let is_reorg = tick % 5 ==0;

            match ingest_block(&pool_task2, block_number, &last_block_hash, is_reorg).await {
                Ok((next_number, next_hash)) =>{
                    block_number = next_number;
                    last_block_hash = next_hash;
                }
                Err(e) => println!("ingest failed at block {block_number}: {e}")
            }

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



