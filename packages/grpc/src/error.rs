use ethers::providers::ProviderError;
use thiserror::Error;

use crate::pb::BlockUpdate;

/// An error that can occur when interacting with the provider.
#[derive(Debug, Error)]
pub enum Error {
    /// An error occurred while interacting with the database.
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Channel send error: {0}")]
    SendError(#[from] tokio::sync::mpsc::error::SendError<BlockUpdate>),

    #[error("Provider error: {0}")]
    ProviderError(#[from] ProviderError),
}
