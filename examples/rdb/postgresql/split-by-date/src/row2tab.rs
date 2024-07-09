use tonic::Status;

use time::Date;
use time::Month;

use rs_split_table::rdb::row2tab::RowToTableName;

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

    pub fn date2string(d: Date) -> String {
        let i: i32 = Self::date2integer(d);
        format!("{i:08}")
    }
}

impl RowToTableName for RowToName {
    type Row = crate::row::Row;
    fn row2table(&self, row: &Self::Row) -> Result<String, Status> {
        let dt: Date = row.date;
        let prefix: &str = self.prefix.as_str();
        let dts: String = Self::date2string(dt);
        Ok(format!("{prefix}{dts}"))
    }
}
