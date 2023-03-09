use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Error {
    pub error: String
}

#[allow(dead_code, clippy::enum_variant_names)]
pub enum Errors {
    SqlError,
    RequestError,
    RateLimitError,
    NotImplementedError
}
