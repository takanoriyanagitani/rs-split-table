use crate::rdb::row2tab::row2tname_new;
use crate::rdb::row2tab::RowToTableName;

pub fn row2tname_row2num_new32i<R, F, S>(row2num: F, num2tab: S) -> impl RowToTableName<Row = R>
where
    R: Send + Sync + 'static,
    F: Fn(&R) -> i32 + Sync + Send + 'static,
    S: Fn(i32) -> String + Sync + Send + 'static,
{
    row2tname_new(move |row: &R| {
        let num: i32 = row2num(row);
        let tab_name: String = num2tab(num);
        Ok(tab_name)
    })
}
