//! Traits to create a table.

use tonic::Status;

/// Creates a table with the specified name.
#[tonic::async_trait]
pub trait CreateTable: Sync + Send + 'static {
    async fn create(&self, name: &str) -> Result<(), Status>;
}
