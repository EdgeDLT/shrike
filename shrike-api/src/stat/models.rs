use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BlockCount {
    pub total_blocks: u64
}

#[derive(Serialize, Deserialize)]
pub struct TransactionCount {
    pub total_transactions: u64
}

#[derive(Serialize, Deserialize)]
pub struct TotalSystemFee {
    pub total_sysfee: f64
}

#[derive(Serialize, Deserialize)]
pub struct InvocationCount {
    pub total_invocations: u64
}

#[derive(Serialize, Deserialize)]
pub struct ContractCount {
    pub total_contracts: u64
}

#[derive(Serialize, Deserialize)]
pub struct TransferCount {
    pub total_transfers: u64
}

#[derive(Serialize, Deserialize)]
pub struct SenderCount {
    pub total_senders: u64
}

#[derive(Serialize, Deserialize)]
pub struct DeployCount {
    pub total_deploys: u64
}
