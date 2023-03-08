use crate::{storage::files_manager, session};
use axum::{
    response::Json,
    routing::{get, post},
    Router, extract::{ContentLengthLimit, Multipart},
};
use axum_database_sessions::AxumSession;
use hyper::Body;
use serde::Deserialize;
use serde_json::{json, Value};

const MAX_UPLOAD_SIZE: u64 = 100 * 1024 * 1024;

pub fn route(app: Router<Body>) -> Router<Body> {
    app.route("/drive/files", get(drive_files))
        .route("/drive/new", post(drive_new))
        .route("/drive/delete", post(drive_delete))
        .route("/drive/upload", post(drive_upload))
        .route("/drive/tree", post(get_file_unpack_tree))
        .route("/drive/ofd", post(drive_ofd))
}

async fn drive_files(session: AxumSession) -> Json<Value> {
    if let Some(user_id) = session::get_user_id(&session).await {
        if let Some(files) = files_manager::load_files(user_id).await {
            if let Ok(v) = serde_json::to_value(files) {
                return Json(json!({
                    "success": true,
                    "payload": v
                }));
            }
        }
    }

    Json(json!({
        "success": false
    }))
}

#[derive(Deserialize)]
struct NewFileBody {
    title: String
}

async fn drive_new(Json(body): Json<NewFileBody>, session: AxumSession) -> Json<Value> {
    if let Some(user_id) = session::get_user_id(&session).await {
        if let Some(file) = files_manager::create_new_file(user_id, &body.title).await {
            if let Ok(v) = serde_json::to_value(file) {
                return Json(json!({
                    "success": true,
                    "payload": v
                }));
            }
        }
    }

    Json(json!({
        "success": false
    }))
}

#[derive(Deserialize)]
pub struct ResFileBody {
    pub id: i64,
}

async fn drive_delete(Json(body): Json<ResFileBody>, session: AxumSession) -> Json<Value> {
    if let Some(user_id) = session::get_user_id(&session).await {
        if files_manager::delete_user_file(user_id, body.id).await {
            return Json(json!({
                "success": true,
                "payload": body.id
            }));
        }
    }

    Json(json!({
        "success": false
    }))
}

async fn drive_upload(ContentLengthLimit(mut multipart): ContentLengthLimit<Multipart, MAX_UPLOAD_SIZE>, session: AxumSession) -> Json<Value> {
    if let Some(user_id) = session::get_user_id(&session).await {
        if let Some(file) = multipart.next_field().await.unwrap() {
            let filename = file.file_name().unwrap().to_string();
            let data = file.bytes().await.unwrap();

            if let Some(file) = files_manager::upload_file(user_id, filename, data).await {
                if let Ok(v) = serde_json::to_value(file) {
                    return Json(json!({
                        "success": true,
                        "payload": v
                    }));
                }
            }
        }

        return Json(json!({
            "success": false
        }));
    }

    Json(json!({
        "success": false
    }))
}

async fn get_file_unpack_tree(Json(body): Json<ResFileBody>) -> Json<Value> {
    let mut json = json!({});
    files_manager::get_file_unpack_tree(body.id, &mut json);

    Json(json!({
        "success": true,
        "payload": json
    }))
}

async fn drive_ofd(Json(body): Json<ResFileBody>) -> Json<Value> {
    files_manager::open_file_folder(body.id).await;
    Json(json!({
        "success": true
    }))
}
