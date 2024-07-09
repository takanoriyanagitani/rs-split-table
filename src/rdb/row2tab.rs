use tonic::Status;

pub trait RowToTableName: Sync + Send + 'static {
    type Row: Send + Sync;

    fn row2table(&self, row: &Self::Row) -> Result<String, Status>;
}
