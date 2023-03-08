use sqlx::{
    sqlite::{SqliteArguments, SqliteRow},
    types::chrono::{DateTime, NaiveDateTime, Utc},
    Arguments, Row,
};

use super::DB;

#[derive(Clone)]
pub struct User {
    pub id: i64,
    pub password: Option<String>,
    pub name: String,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub nick: Option<String>,
    pub create_time: DateTime<Utc>,
}

impl User {
    pub fn new() -> Self {
        Self {
            id: -1,
            password: None,
            name: String::default(),
            mobile: None,
            email: None,
            nick: None,
            create_time: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
        }
    }
}

const USER_QUERY_FIELD: &str = "id,password,name,mobile,email,nick,create_time";
impl DB {
    fn user_get_by_row(row: &SqliteRow) -> User {
        let mut user = User::new();
        user.id = row.get(0);
        user.password = row.get(1);
        user.name = row.get(2);
        user.mobile = row.get(3);
        user.email = row.get(4);
        user.nick = row.get(5);
        user.create_time = row.get(6);

        user
    }
}

impl DB {
    pub async fn user_migrate(&self) {
        match sqlx::query(
            r#"
        CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            password TEXT NULL,
            mobile TEXT NULL UNIQUE,
            email TEXT NULL UNIQUE,
            nick TEXT NULL,
            create_time INTEGER NOT NULL
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

    pub async fn user_insert(&self, mut user: User) -> Option<User> {
        let now = DateTime::<Utc>::from_utc(NaiveDateTime::from(Utc::now().naive_utc()), Utc);

        let mut field = String::from("name,create_time");
        let mut argument = String::from("?,?");
        let mut values = SqliteArguments::default();
        values.add(&user.name);
        values.add(now.timestamp());

        if let Some(password) = user.password.as_ref() {
            field.push_str(",password");
            argument.push_str(",?");
            values.add(password);
        }
        if let Some(mobile) = user.mobile.as_ref() {
            field.push_str(",mobile");
            argument.push_str(",?");
            values.add(mobile);
        }
        if let Some(email) = user.email.as_ref() {
            field.push_str(",email");
            argument.push_str(",?");
            values.add(email);
        }
        if let Some(nick) = user.nick.as_ref() {
            field.push_str(",nick");
            argument.push_str(",?");
            values.add(nick);
        }

        let sql = format!("INSERT INTO user ({}) VALUES ({})", field, argument);
        match sqlx::query_with(&sql, values).execute(&self.pool).await {
            Ok(result) => {
                user.id = result.last_insert_rowid();
                user.create_time = now;
                Some(user)
            }
            Err(err) => {
                println!("{}", err);
                None
            }
        }
    }

    pub async fn user_query_by_name(&self, name: &str) -> Option<User> {
        match sqlx::query(&format!("SELECT {} FROM user WHERE name = ?", USER_QUERY_FIELD))
            .bind(name)
            .fetch_optional(&self.pool)
            .await
        {
            Ok(result) => {
                if let Some(row) = result {
                    Some(Self::user_get_by_row(&row))
                } else {
                    println!("user: \"{}\" can not found!", name);
                    None
                }
            }
            Err(err) => {
                println!("{}", err);
                None
            }
        }
    }

    pub async fn user_exist_by_id(&self, user_id: i64) -> Option<bool> {
        match sqlx::query("SELECT 1 FROM user WHERE id = ? LIMIT 1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
        {
            Ok(result) => {
                if let Some(_row) = result {
                    Some(true)
                } else {
                    Some(false)
                }
            }
            Err(err) => {
                println!("{}", err);
                None
            }
        }
    }
}
