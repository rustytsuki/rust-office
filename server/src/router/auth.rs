use axum::{
    response::{IntoResponse, Json, Redirect, Response},
    routing::{get, post},
    Router,
};
use axum_database_sessions::AxumSession;
use hyper::{header::LOCATION, Body, StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use sha_crypt::{sha512_check, sha512_simple, Sha512Params};

use crate::{
    db::{user::{User, self}, DB},
    session,
};

pub fn route(app: Router<Body>) -> Router<Body> {
    app.route("/auth/signup", post(signup))
        .route("/auth/signin", post(signin))
        .route("/auth/signout", post(signout))
        .route("/auth/session.js", get(session))
}

#[derive(Deserialize)]
pub struct SignUpBody {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "password")]
    pub password: String,
}

async fn signup(Json(body): Json<SignUpBody>, session: AxumSession) -> Json<Value> {
    if session::get_user_id(&session).await.is_some() {
        return Json(json!({
            "success": false
        }));
    }

    let name = body.name.trim();
    let password = body.password.trim();
    if name.is_empty() || password.is_empty() {
        return Json(json!({
            "success": false
        }));
    }

    // Hash the password for storage
    let params = Sha512Params::new(10_000).expect("RandomError!");
    let hashed_password = sha512_simple(&password, &params).expect("Should not fail");

    let mut user = User::new();
    user.name = name.to_string();
    user.password = Some(hashed_password);

    if let Some(user) = DB::inst().user_insert(user).await {
        return Json(json!({
            "success": true,
            "payload": user.name,
        }));
    }

    Json(json!({
        "success": false
    }))
}

#[derive(Deserialize)]
pub struct SignInBody {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "password")]
    pub password: String,
}

async fn signin(Json(body): Json<SignInBody>, session: AxumSession) -> Json<Value> {
    let name = body.name.trim();
    let password = body.password.trim();

    if let Some(user) = DB::inst().user_query_by_name(name).await {
        if let Some(hashed_password) = user.password.as_ref() {
            // check password
            if sha512_check(password, hashed_password).is_ok() {
                session::set_user_id(&session, user.id).await;
                session::set_user_name(&session, &user.name).await;

                return Json(json!({
                    "success": true
                }));
            }
        }
    }

    Json(json!({
        "success": false
    }))
}

async fn signout(session: AxumSession) -> Json<Value> {
    session::remove_user_id(&session).await;
    session::remove_user_name(&session).await;
    // session.destroy().await;

    return Json(json!({
        "success": true
    }));
}

async fn session(session: AxumSession) -> impl IntoResponse {
    let mut script = String::from("window.session={");
    if let Some(user_id) = session::get_user_id(&session).await {
        // get user
        script.push_str("user:{");
        script.push_str(&format!("id:'{}'", user_id));
        if let Some(user_name) = session::get_user_name(&session).await {
            script.push_str(&format!(",name:'{}'", user_name));
        }
        script.push_str("}");
    }
    script.push_str("};");

    (StatusCode::OK, [("content-type", "application/javascript")], script)
}
