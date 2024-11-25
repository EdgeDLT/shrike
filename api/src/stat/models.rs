use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BlockCount {
    pub total_blocks: u64,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionCount {
    pub total_transactions: u64,
}

#[derive(Serialize, Deserialize)]
pub struct TotalSystemFee {
    pub total_sysfee: f64,
}

#[derive(Serialize, Deserialize)]
pub struct TransferCount {
    pub total_transfers: u64,
}

#[derive(Serialize, Deserialize)]
pub struct SenderCount {
    pub total_senders: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ContractCount {
    pub total_contracts: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ShrikeStats {
    pub total_blocks: u64,
    pub total_transactions: u64,
    pub total_sysfee: f64,
    pub total_transfers: u64,
    pub total_senders: u64,
    pub total_contracts: u64,
}

#[derive(Serialize, Deserialize)]
pub struct NetworkStatistics {
    pub total_transactions: u64,
    pub total_addresses: u64,
    pub total_contracts: u64,
    pub current_week_transactions: u64,
    pub current_week_addresses: u64,
    pub current_week_contracts: u64,
}
