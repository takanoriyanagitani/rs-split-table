use tonic::Status;

use crate::rdb::create::CreateTable;
use crate::rdb::insert::Insert;
use crate::rdb::row2tab::RowToTableName;
use crate::rdb::tabchk::TableChecker;

use crate::target::DataTarget;

pub struct RdbSaver<C, I, T, R> {
    create: C,
    insert: I,
    tabchk: T,
    row2nm: R,
}

#[tonic::async_trait]
impl<C, I, T, R> DataTarget for RdbSaver<C, I, T, R>
where
    C: CreateTable,
    I: Insert,
    T: TableChecker,
    R: RowToTableName<Row = I::Row>,
{
    type Row = I::Row;

    async fn save(&self, row: &Self::Row) -> Result<(), Status> {
        let unchecked_tabname: String = self.row2nm.row2table(row)?;
        let safe_tabname: String = self.tabchk.to_checked(unchecked_tabname)?;
        self.create.create(safe_tabname.as_str()).await?;
        self.insert
            .insert_to_table(safe_tabname.as_str(), row)
            .await?;
        Ok(())
    }
}

pub fn saver_new<C, I, T, R>(
    create: C,
    insert: I,
    tabchk: T,
    row2nm: R,
) -> impl DataTarget<Row = I::Row>
where
    C: CreateTable,
    I: Insert,
    T: TableChecker,
    R: RowToTableName<Row = I::Row>,
{
    RdbSaver {
        create,
        insert,
        tabchk,
        row2nm,
    }
}
