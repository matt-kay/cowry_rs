#[derive(Debug, thiserror::Error)]
pub enum OwoError {
    #[error("Currency mismatch: {0} vs {1}")]
    CurrencyMismatch(String, String),

    #[error("Division by zero is not allowed")]
    DivisionByZero,

    #[error("Invalid JSON: {0}")]
    SerdeError(#[from] serde_json::Error),
}