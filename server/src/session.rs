use axum_database_sessions::AxumSession;

pub async fn set_user_id(session: &AxumSession, user_id: i64) {
    session.set("user-id", user_id).await;
}

pub async fn remove_user_id(session: &AxumSession) {
    session.remove("user-id").await;
}

pub async fn get_user_id(session: &AxumSession) -> Option<i64> {
    session.get::<i64>("user-id").await
}

pub async fn set_user_name(session: &AxumSession, user_name: &str) {
    session.set("user-name", user_name).await;
}

pub async fn remove_user_name(session: &AxumSession) {
    session.remove("user-name").await;
}

pub async fn get_user_name(session: &AxumSession) -> Option<String> {
    session.get::<String>("user-name").await
}