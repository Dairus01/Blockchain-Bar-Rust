use thiserror::Error;
#[derive(Error, Debug)]
pub enum MempoolError{
    #[error("Transaction is invalid: {reason}")]
    InvalidTransaction { reason: String },

    #[error("Transaction already exists in the mempool.")]
    AlreadyExists,

    #[error("Mempool is full and cannot accept new transactions.")]
    MempoolFull,

    #[error("Database error while checking state")]
    DatabaseError, // Automatically implements conversion

    #[error("Applying of tx to state was unsuccessful")]
    ApplyTxUnsuccessful,

    #[error("Could not persist to disk")]
    WriteAllUnsuccessful,
}