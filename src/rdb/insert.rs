use tonic::Status;

#[tonic::async_trait]
pub trait Insert: Sync + Send + 'static {
    type Row: Send + Sync;
    async fn insert_to_table(
        &self,
        checked_table_name: &str,
        row: &Self::Row,
    ) -> Result<u64, Status>;
}
