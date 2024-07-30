//! Traits for creating table names.

use core::marker::PhantomData;

use tonic::Status;

/// Tries to generate a table name from a row.
pub trait RowToTableName: Sync + Send + 'static {
    type Row: Send + Sync;

    fn row2table(&self, row: &Self::Row) -> Result<String, Status>;
}

pub struct RowToTabNameFn<R, F> {
    orow: PhantomData<R>,
    row2tab: F,
}

impl<R, F> RowToTableName for RowToTabNameFn<R, F>
where
    F: Fn(&R) -> Result<String, Status> + Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    type Row = R;

    fn row2table(&self, row: &Self::Row) -> Result<String, Status> {
        (self.row2tab)(row)
    }
}

pub fn row2tname_new<R, F>(row2tab: F) -> impl RowToTableName<Row = R>
where
    F: Fn(&R) -> Result<String, Status> + Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    RowToTabNameFn {
        orow: PhantomData::default(),
        row2tab,
    }
}
