use deadpool_postgres::Pool;

use tonic::Status;

use rs_split_table::rdb::insert::Insert;

use crate::row::Row;

pub struct InsPg {
    pool: Pool,
}

impl InsPg {
    pub async fn insert_trusted(&self, trusted_table_name: &str, row: &Row) -> Result<u64, Status> {
        let sql = format!(
            r#"
			    INSERT INTO {trusted_table_name}
			    VALUES (
			    	$1::TEXT,
			    	$2::DATE,
			    	$3::TEXT
			    )
		    "#
        );
        let cli = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(format!("unable to get a client: {e}")))?;
        cli.execute(&sql, &[&row.id, &row.date, &row.msg])
            .await
            .map_err(|e| Status::internal(format!("unable to insert: {e}")))
    }
}

#[tonic::async_trait]
impl Insert for InsPg {
    type Row = crate::row::Row;

    async fn insert_to_table(
        &self,
        checked_table_name: &str,
        row: &Self::Row,
    ) -> Result<u64, Status> {
        self.insert_trusted(checked_table_name, row).await
    }
}

pub fn saver_new(pool: Pool) -> impl Insert<Row = crate::row::Row> {
    InsPg { pool }
}
