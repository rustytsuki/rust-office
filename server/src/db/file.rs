use sqlx::{
    sqlite::{SqliteArguments, SqliteRow},
    types::chrono::{DateTime, NaiveDateTime, Utc},
    Arguments, Row,
};

use super::DB;

#[derive(Clone)]
pub struct File {
    pub id: i64,
    pub user: i64,
    pub title: String,
    pub create_time: DateTime<Utc>,
    pub modify_time: DateTime<Utc>,
}

impl File {
    pub fn new() -> Self {
        Self {
            id: -1,
            user: -1,
            title: String::default(),
            create_time: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            modify_time: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
        }
    }
}

const FILE_QUERY_FIELD: &str = "id,user,title,create_time,modify_time";
impl DB {
    fn file_get_by_row(row: &SqliteRow) -> File {
        let mut file = File::new();
        file.id = row.get(0);
        file.user = row.get(1);
        file.title = row.get(2);
        file.create_time = row.get(3);
        file.modify_time = row.get(4);

        file
    }
}

impl DB {
    pub async fn file_migrate(&self) {
        match sqlx::query(
            r#"
        CREATE TABLE IF NOT EXISTS file (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user INTEGER NOT NULL,
            title TEXT NOT NULL,
            create_time INTEGER NOT NULL,
            modify_time INTEGER NOT NULL
        )"#,
        )
        .execute(&self.pool)
        .await
        {
            Ok(_) => {}
            Err(err) => {
                println!("{}", err);
            }
        }
    }

    pub async fn file_insert(&self, user_id: i64, title: &str) -> Option<File> {
        let now = DateTime::<Utc>::from_utc(NaiveDateTime::from(Utc::now().naive_utc()), Utc);

        let field = String::from("user,title,create_time,modify_time");
        let argument = String::from("?,?,?,?");
        let mut values = SqliteArguments::default();
        values.add(user_id);
        values.add(title);
        values.add(now.timestamp());
        values.add(now.timestamp());

        let sql = format!("INSERT INTO file ({}) VALUES ({})", field, argument);
        match sqlx::query_with(&sql, values).execute(&self.pool).await {
            Ok(result) => {
                let mut file = File::new();
                file.id = result.last_insert_rowid();
                file.user = user_id;
                file.title = title.to_string();
                file.create_time = now;
                file.modify_time = now;
                Some(file)
            }
            Err(err) => {
                println!("file_insert error: {}", err);
                None
            }
        }
    }

    pub async fn file_delete_by_user(&self, user_id: i64, file_id: i64) -> bool {
        match sqlx::query("DELETE FROM file WHERE id = ? AND user = ?")
            .bind(file_id)
            .bind(user_id)
            .execute(&self.pool)
            .await
        {
            Ok(_result) => true,
            Err(err) => {
                println!("{}", err);
                false
            }
        }
    }

    pub async fn file_query_by_user(&self, user_id: i64) -> Option<Vec<File>> {
        match sqlx::query(&format!(
            "SELECT {} FROM file WHERE user = ? ORDER BY modify_time DESC",
            FILE_QUERY_FIELD
        ))
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        {
            Ok(result) => {
                let mut files: Vec<File> = Vec::new();
                for row in result {
                    files.push(Self::file_get_by_row(&row));
                }
                Some(files)
            }
            Err(err) => {
                println!("{}", err);
                None
            }
        }
    }

    pub async fn file_query_by_id(&self, id: i64) -> Option<File> {
        match sqlx::query(&format!("SELECT {} FROM file WHERE id = ?", FILE_QUERY_FIELD))
            .bind(id)
            .fetch_optional(&self.pool)
            .await
        {
            Ok(result) => {
                if let Some(row) = result {
                    Some(Self::file_get_by_row(&row))
                } else {
                    None
                }
            }
            Err(err) => {
                println!("{}", err);
                None
            }
        }
    }

    pub async fn file_query_seq_by_id(&self, id: i64) -> Option<i64> {
        match sqlx::query("SELECT seq FROM file WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
        {
            Ok(result) => {
                if let Some(row) = result {
                    Some(row.get(0))
                } else {
                    None
                }
            }
            Err(err) => {
                println!("{}", err);
                None
            }
        }
    }

    pub async fn file_update_seq_by_id(&self, id: i64, seq: i64) -> bool {
        match sqlx::query("UPDATE file SET seq = ? WHERE id = ?")
            .bind(seq)
            .bind(id)
            .execute(&self.pool)
            .await {
                Ok(_) => {
                    return true;
                },
                Err(err) => {
                    println!("{}", err);
                },
            }
        false
    }
}
