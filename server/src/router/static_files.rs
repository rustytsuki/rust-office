use axum::{http::StatusCode, routing::get_service, routing::MethodRouter, Router};
use hyper::Body;
use tower_http::services::ServeDir;

pub fn route(app: Router<Body>) -> Router<Body> {
    app.nest("/storage", get_service_static_file("./data/storage/files"))
}

pub fn get_service_static_file(path: &str) -> MethodRouter {
    get_service(ServeDir::new(path)).handle_error(|error: std::io::Error| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", error),
        )
    })
}
