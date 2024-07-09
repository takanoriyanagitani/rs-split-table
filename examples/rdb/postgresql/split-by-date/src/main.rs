use std::process::ExitCode;

use tonic::Status;

use tokio_postgres::{Config, NoTls};

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};

use split_by_date::row2tab::RowToName;
use split_by_date::tabchk::table_checker_prefix_integer_new;

use split_by_date::create::creator_new;
use split_by_date::insert::saver_new;
use split_by_date::row::Row;

const TABLE_PREFIX: &str = "testtab";

async fn sub() -> Result<(), Status> {
    // RowToTableName
    let row2tab = RowToName {
        prefix: TABLE_PREFIX.into(),
    };

    // TableChecker
    let tabchk = table_checker_prefix_integer_new(TABLE_PREFIX.into());

    let mut conf: Config = Config::new();
    conf.port(5432);
    conf.hostaddr([127, 0, 0, 1].into());
    conf.hostaddr([127, 0, 0, 1].into());
    conf.user("postgres");

    let mconf = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(conf, NoTls, mconf);
    let pool = Pool::builder(mgr)
        .max_size(2)
        .build()
        .map_err(|e| Status::internal(format!("unable to create pg pool: {e}")))?;

    // CreateTable
    let creator = creator_new(pool.clone());

    // Insert
    let insert = saver_new(pool.clone());

    // DataTarget
    let saver = rs_split_table::rdb::saver::saver_new(creator, insert, tabchk, row2tab);

    // DataSource
    let source = split_by_date::source::DummySource {
        rows: vec![

            Row {
                id: "cafef00d-dead-beaf-face-864299792458".into(),
                date: time::macros::date!(2024 - 07 - 09),
                msg: "helo0".into(),
            },
            Row {
                id: "cafef00d-dead-beaf-face-864299792458".into(),
                date: time::macros::date!(2024 - 07 - 09),
                msg: "helo1".into(),
            },

            Row {
                id: "cafef00d-dead-beaf-face-864299792458".into(),
                date: time::macros::date!(2024 - 07 - 10),
                msg: "helo0".into(),
            },
            Row {
                id: "cafef00d-dead-beaf-face-864299792458".into(),
                date: time::macros::date!(2024 - 07 - 10),
                msg: "helo1".into(),
            },

        ],
    };

    let inserted: u64 = rs_split_table::split::copy(&source, &saver).await?;
    println!("inserted: {inserted}");

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    sub().await.map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
