use crate::router::static_files::get_service_static_file;
use axum::{
    response::{Html, Redirect},
    routing::get,
    Router,
};
use hyper::Body;
use std::path::PathBuf;
use tokio::fs;

const PAGES_PATH: &str = "./data/resource/pages";

pub fn route(app: Router<Body>) -> Router<Body> {
    let path: PathBuf = [PAGES_PATH, "_next"].iter().collect();
    app.nest("/rusty/_next", get_service_static_file(path.to_str().unwrap()))
        .fallback(get(|| async { Redirect::permanent("/rusty/404") }))
}

pub async fn get_page(arr: &[&str]) -> Html<String> {
    let mut path = PathBuf::from(PAGES_PATH);
    for v in arr.iter() {
        path.push(v);
    }
    let html = fs::read(path).await.unwrap();
    let html = String::from_utf8(html).unwrap();
    Html(crate::router::pages::handle_html(html))
}
