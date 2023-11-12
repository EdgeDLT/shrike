use serde::Deserialize;

use super::models::{AppLogResult, BlockResult, NeoParam};

pub trait RpcMethod {
    type ReturnType: for<'de> Deserialize<'de>;

    fn method_name(&self) -> &'static str;
    fn params(&self) -> Vec<NeoParam>;
}

pub struct GetBlockCount;

impl RpcMethod for GetBlockCount {
    type ReturnType = u64;

    fn method_name(&self) -> &'static str {
        "getblockcount"
    }

    fn params(&self) -> Vec<NeoParam> {
        vec![]
    }
}

pub struct GetBlock {
    pub block_height: u64,
    pub verbosity: u8,
}

impl RpcMethod for GetBlock {
    type ReturnType = BlockResult;

    fn method_name(&self) -> &'static str {
        "getblock"
    }

    fn params(&self) -> Vec<NeoParam> {
        vec![
            NeoParam::Integer(self.block_height),
            NeoParam::Integer(self.verbosity as u64),
        ]
    }
}

pub struct GetApplicationLog {
    pub hash: String,
}

impl RpcMethod for GetApplicationLog {
    type ReturnType = AppLogResult;

    fn method_name(&self) -> &'static str {
        "getapplicationlog"
    }

    fn params(&self) -> Vec<NeoParam> {
        vec![NeoParam::String(self.hash.clone())]
    }
}
