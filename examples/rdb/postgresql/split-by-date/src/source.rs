use futures::StreamExt;
use futures::TryStreamExt;

use tokio_stream::wrappers::ReceiverStream;

use tonic::Status;

use rs_split_table::source::DataSource;

use crate::row::Row;

pub struct DummySource {
    pub rows: Vec<Row>,
}

#[tonic::async_trait]
impl DataSource for DummySource {
    type Row = crate::row::Row;

    type Rows = ReceiverStream<Result<Self::Row, Status>>;

    async fn get_rows(&self) -> Result<Self::Rows, Status> {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        let rows: Vec<_> = self.rows.clone();
        tokio::spawn(async move {
            let t: &_ = &tx;
            let strm = futures::stream::iter(rows);
            let mapd = strm.map(Ok);
            let _cnt: Result<u64, Status> = mapd
                .try_fold(0, |state, next| async move {
                    t.send(Ok(next))
                        .await
                        .map_err(|e| Status::internal(format!("unable to send a row: {e}")))?;
                    Ok(state)
                })
                .await;
            match _cnt {
                Ok(_) => {}
                Err(e) => eprintln!("{e}"),
            }
        });
        Ok(ReceiverStream::new(rx))
    }
}
