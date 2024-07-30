use time::Date;
use time::Month;

use rs_split_table::rdb::row2tab::RowToTableName;
use rs_split_table::rdb::simple::row2tab::row2num::row2tname_row2num_new32i;

pub struct RowToName {
    pub prefix: String,
}

impl RowToName {
    pub fn date2integer(d: Date) -> i32 {
        let y: i32 = d.year();

        let m: Month = d.month();
        let mu: u8 = m.into(); // 1,2,3, ... , 12
        let mi: i32 = mu.into();

        let d: i32 = d.day().into();

        let ymd: i32 = y * 10_000 + mi * 100 + d;
        ymd
    }

    pub fn row2num(row: &crate::row::Row) -> i32 {
        let dt: Date = row.date;
        Self::date2integer(dt)
    }

    pub fn num2tab(&self, num: i32) -> String {
        let prefix: &str = self.prefix.as_str();
        format!("{prefix}{num:08}")
    }

    pub fn into_num2tab(self) -> impl Fn(i32) -> String {
        move |num: i32| self.num2tab(num)
    }

    pub fn into_row2tab(self) -> impl RowToTableName<Row = crate::row::Row> {
        row2tname_row2num_new32i(Self::row2num, self.into_num2tab())
    }
}
