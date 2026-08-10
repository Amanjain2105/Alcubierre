use serde::{Deserialize,Serialize};

#[derive(Serialize,Debug)]
pub struct AccountInfo{
    pub balance: String,
    pub nonce: u64,
}