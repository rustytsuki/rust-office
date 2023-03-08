pub mod file;
pub mod user;

use lazy_static::lazy_static;
use std::{path::PathBuf, str::FromStr, sync::RwLock};

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};

const SQL_LITE_DB_PATH: &str = "./data/storage/db";
const SQL_LITE_DB_FILE: &str = "data.db";

lazy_static!{
    static ref DATA_BASE: RwLock<Option<DB>> = RwLock::new(None);
}

#[derive(Clone)]
pub struct DB {
    pub pool: Pool<Sqlite>,
}

impl DB {
    pub fn inst() -> DB {
        let db = DATA_BASE.read().unwrap();
        let db = db.as_ref().unwrap().clone();
        db
    }

    pub async fn connect() {
        let mut db_path = PathBuf::from(SQL_LITE_DB_PATH);
        crate::utils::ensure_dir(&db_path);
        db_path.push(SQL_LITE_DB_FILE);

        let mut sql_lite_uri = String::from("sqlite://");
        sql_lite_uri += db_path.as_os_str().to_str().unwrap();
        let opt = SqliteConnectOptions::from_str(&sql_lite_uri)
            .unwrap()
            .create_if_missing(true)
            .read_only(false);

        match SqlitePoolOptions::new().max_connections(1).connect_with(opt).await {
            Ok(pool) => {
                let mut db = DATA_BASE.write().unwrap();
                *db = Some(Self { pool });
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    pub async fn migrate(&self) {
        self.user_migrate().await;
        self.file_migrate().await;
    }
}
