use crate::rdb::row2tab::row2tname_new;
use crate::rdb::row2tab::RowToTableName;

pub fn row2tname_row2cp_new<R, F, S, C>(row2cp: F, cp2tab: S) -> impl RowToTableName<Row = R>
where
    R: Send + Sync + 'static,
    C: Copy,
    F: Fn(&R) -> C + Sync + Send + 'static,
    S: Fn(C) -> String + Sync + Send + 'static,
{
    row2tname_new(move |row: &R| {
        let cp: C = row2cp(row);
        let tab_name: String = cp2tab(cp);
        Ok(tab_name)
    })
}
