use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Balance {
    pub sub: String,
    pub amount: u32,
}
