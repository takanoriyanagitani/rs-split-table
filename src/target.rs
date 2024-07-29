//! Traits of data savers.

use tonic::Status;

/// Tries to save a row.
#[tonic::async_trait]
pub trait DataTarget: Sync + Send + 'static {
    type Row;

    async fn save(&self, row: &Self::Row) -> Result<(), Status>;
}
