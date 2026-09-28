use crate::{AppState, types::account::*};
use axum::extract::State;
use sqlx::Row;


pub async fn get_account_info(State(state): State<AppState>, address: String) -> Result<AccountInfo, String>{

    let row = sqlx::query("SELECT balance, nonce FROM accounts WHERE address = $1")
    .bind(&address)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| e.to_string())?;

    match row {
        Some(row) => Ok(AccountInfo{
            balance: row.get("balance"),
            nonce: row.get("nonce"),
        }),
        None => Err("Account Not Found".to_string()),
    }
    
}

pub async fn get_supported_tokens(chain_id: String) -> Result<Vec<Token>, String>{
    Ok(vec![
        Token { 
            token_id: "1".to_string(), 
            symbol: "ETH".to_string(), 
            contract_address: "0xabc".to_string() 
        },
        Token { 
            token_id: "2".to_string(), 
            symbol: "USDC".to_string(), 
            contract_address: "0xdef".to_string() 
        },
    ])
}

pub async fn get_token_balances(address: String, chain_id: String) -> Result<Vec<TokenBalance>, String>{
    Ok(vec![
        TokenBalance {
            token_id: "1".to_string(),
            symbol: "ETH".to_string(),
            balance: "1".to_string()
        },
        TokenBalance {
            token_id: "2".to_string(),
            symbol: "USDC".to_string(),
            balance: "3".to_string()
        }
    ])

}