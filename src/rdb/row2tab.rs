use tonic::Status;

/// Tries to generate a table name from a row.
pub trait RowToTableName: Sync + Send + 'static {
    type Row: Send + Sync;

    fn row2table(&self, row: &Self::Row) -> Result<String, Status>;
}
