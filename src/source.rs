use futures::Stream;

use tonic::Status;

/// Tries to get rows.
#[tonic::async_trait]
pub trait DataSource: Sync + Send + 'static {
    type Row;
    type Rows: Stream<Item = Result<Self::Row, Status>>;

    async fn get_rows(&self) -> Result<Self::Rows, Status>;
}
