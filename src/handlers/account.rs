use crate::types::account::AccountInfo;


pub async fn get_account_info(address: String) -> Result<AccountInfo, String>{
    Ok(AccountInfo{
        balance: "1000000".to_string(),
        nonce: 0,
    })
}