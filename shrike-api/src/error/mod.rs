use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Error {
    pub error: String
}

#[allow(dead_code)]
pub enum Errors {
    SqlError,
    RequestError,
    RateLimitError,
    NotImplementedError
}
