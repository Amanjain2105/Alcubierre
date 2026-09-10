use crate::types::account::*;


pub async fn get_account_info(address: String) -> Result<AccountInfo, String>{
    Ok(AccountInfo{
        balance: "1000000".to_string(),
        nonce: 0,
    })
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