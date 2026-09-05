use serde::{Serialize};

#[derive(Serialize, PartialEq)]
#[serde(tag ="status")]
pub enum TransactionStatus {
    Received,
    Pending { queue_position: u64 },
    Included { block_number: u64 },
    Finalized { block_number: u64, gas_used: u64 },
    Failed { reason: String, gas_used: Option<u64> }
}



