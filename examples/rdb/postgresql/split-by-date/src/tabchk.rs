use tonic::Status;

use rs_split_table::rdb::tabchk::fn2checker;
use rs_split_table::rdb::tabchk::TableChecker;

pub fn table_checker_prefix_integer_new(prefix: String) -> impl TableChecker + Clone {
    fn2checker(move |tabname: &str| {
        let pchk: bool = tabname.starts_with(prefix.as_str());
        pchk.then_some(()).ok_or_else(|| {
            Status::invalid_argument(format!("invalid table name(no prefix): {tabname}"))
        })?;
        let replaced: String = tabname.replacen(prefix.as_str(), "", 1);
        let _: u32 = str::parse(replaced.as_str()).map_err(|_| {
            Status::invalid_argument(format!(
                "invalid table name(expected an integer): {tabname}"
            ))
        })?;
        Ok(())
    })
}
