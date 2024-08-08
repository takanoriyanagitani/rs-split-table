//! Functions to create [`RowToTableName`].

use crate::rdb::row2tab::RowToTableName;

use super::row2cp::row2tname_row2cp_new;

macro_rules! row2tname_row2num {
    ($numtyp: ty, $fname: ident) => {
        /// Creates a [`RowToTableName`] which uses a number to create the name.
        ///
        /// ## Arguments
        /// - row2num: Converts the row to the number.
        /// - num2tab: Creates the name of the table from the number.
        pub fn $fname<R, F, S>(row2num: F, num2tab: S) -> impl RowToTableName<Row = R>
        where
            R: Send + Sync + 'static,
            F: Fn(&R) -> $numtyp + Sync + Send + 'static,
            S: Fn($numtyp) -> String + Sync + Send + 'static,
        {
            row2tname_row2cp_new(row2num, num2tab)
        }
    };
}

row2tname_row2num!(i128, row2tname_row2num_new128i);
row2tname_row2num!(i64, row2tname_row2num_new64i);
row2tname_row2num!(i32, row2tname_row2num_new32i);
row2tname_row2num!(i16, row2tname_row2num_new16i);
row2tname_row2num!(i8, row2tname_row2num_new8i);

row2tname_row2num!(u128, row2tname_row2num_new128u);
row2tname_row2num!(u64, row2tname_row2num_new64u);
row2tname_row2num!(u32, row2tname_row2num_new32u);
row2tname_row2num!(u16, row2tname_row2num_new16u);
row2tname_row2num!(u8, row2tname_row2num_new8u);
