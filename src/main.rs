pub mod extractor;
mod types;
mod handlers;

use axum::{routing::post, Router};
use handlers::rpc;






// --------------------------------------------------
// Main
// --------------------------------------------------

#[tokio::main]
async fn main() {
    let app = Router::new().route("/rpc", post(rpc));

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