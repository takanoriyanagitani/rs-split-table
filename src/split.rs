//! Functions to copy the rows from the source to the target.

use futures::TryStreamExt;

use tonic::Status;

use crate::source::DataSource;
use crate::target::DataTarget;

/// Copies the rows got from the source to the target.
pub async fn copy<S, T>(src: &S, tgt: &T) -> Result<u64, Status>
where
    S: DataSource,
    T: DataTarget<Row = S::Row>,
{
    let row_strm = src.get_rows().await?;
    row_strm
        .try_fold(0, |state, next| async move {
            let row: &S::Row = &next;
            tgt.save(row).await?;
            Ok(state + 1)
        })
        .await
}
