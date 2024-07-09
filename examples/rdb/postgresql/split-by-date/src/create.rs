use deadpool_postgres::Pool;

use tonic::Status;

use rs_split_table::rdb::create::CreateTable;

pub struct CreatePg {
    pool: Pool,
}

impl CreatePg {
    pub async fn create_trusted(&self, trusted_table_name: &str) -> Result<(), Status> {
        let sql = format!(
            r#"
			    CREATE TABLE IF NOT EXISTS {trusted_table_name} (
				  id TEXT NOT NULL,
				  date DATE NOT NULL,
				  msg TEXT NOT NULL
			    )
		    "#
        );
        let cli = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(format!("unable to get a client: {e}")))?;
        cli.execute(&sql, &[])
            .await
            .map(|_| ())
            .map_err(|e| Status::internal(format!("unable to insert: {e}")))
    }
}

#[tonic::async_trait]
impl CreateTable for CreatePg {
    async fn create(&self, name: &str) -> Result<(), Status> {
        self.create_trusted(name).await
    }
}

pub fn creator_new(pool: Pool) -> impl CreateTable {
    CreatePg { pool }
}
