use serde::{Deserialize, Serialize};

/// A representation of a currency, such as USD or NGN.
#[derive(Serialize, Deserialize, Debug, Clone,PartialEq,Eq)]
pub struct Currency {
    pub code: String,
    pub symbol: String,
    pub precision: u8,
}

impl Currency {
    /// Creates a new currency definition.
    pub fn new(code: &str, symbol: &str, precision: u8) -> Self {
        Self {
            code: code.to_string(),
            symbol: symbol.to_string(),
            precision,
        }
    }
}
