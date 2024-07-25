//! Functions to create [`TableChecker`]s(ok pattern: prefix + number).
use tonic::Status;

use crate::rdb::tabchk::{double_checker_new, TableChecker};

macro_rules! prefix_checker_new {
    ($fname: ident, $typname: ty) => {
        /// Checks if the name starts with the prefix followed by an integer.
        pub fn $fname(prefix: String) -> impl TableChecker {
            let p1: String = prefix.clone();
            let p2: String = prefix.clone();
            double_checker_new(
                move |unchecked: &str| {
                    let ok: bool = unchecked.starts_with(p1.as_str());
                    ok.then_some(()).ok_or_else(|| {
                        Status::invalid_argument(format!("invalid table name(prefix): {unchecked}"))
                    })
                },
                move |unchecked: &str| {
                    let noprefix: String = unchecked.replacen(p2.as_str(), "", 1);
                    let _: $typname = str::parse(noprefix.as_str()).map_err(|e| {
                        Status::invalid_argument(format!("invalid table name({unchecked}): {e}"))
                    })?;
                    Ok(())
                },
            )
        }
    };
}

prefix_checker_new!(prefix_checker16u, u16);
prefix_checker_new!(prefix_checker32u, u32);
prefix_checker_new!(prefix_checker64u, u64);
