use serde::{Deserialize,Serialize};

#[derive(Serialize,Debug)]
pub struct AccountInfo{
    pub balance: String,
    pub nonce: u64,
}
#[derive(Debug,Serialize)]
pub struct TokenBalance{
    pub token_id: String,
    pub symbol: String,
    pub balance: String
}


#[derive(Debug,Serialize)]
pub struct Token{
    pub token_id: String,
    pub symbol: String,
    pub contract_address: String
}