mod reverse_proxy;
mod static_pages;

use crate::{config};
use axum::extract::Extension;
use axum::response::{Html, Redirect};
use axum::routing::get;
use axum::Router;
use hyper::client::HttpConnector;
use hyper::{Body, Request};
type Client = hyper::client::Client<HttpConnector, Body>;

pub fn route(app: Router<Body>) -> Router<Body> {
    let app = app
        .route("/", get(|| async { Redirect::permanent("/rusty/template") }))
        .route("/rusty/signin", get(signin))
        .route("/rusty/signup", get(signup))
        .route("/rusty/template", get(template))
        .route("/rusty/drive", get(drive))
        .route("/rusty/edit/:fid", get(edit))
        .route("/rusty/about", get(about))
        .route("/rusty/404", get(not_found));

    let cfg = config::inst().read().unwrap();
    if cfg.default.proxy {
        return reverse_proxy::route(app);
    } else {
        return static_pages::route(app);
    }
}

async fn signin(Extension(client): Extension<Client>, req: Request<Body>) -> Html<String> {
    get_page(&["signin.html"], Extension(client), req).await
}

async fn signup(Extension(client): Extension<Client>, req: Request<Body>) -> Html<String> {
    get_page(&["signup.html"], Extension(client), req).await
}

async fn template(Extension(client): Extension<Client>, req: Request<Body>) -> Html<String> {
    get_page(&["template.html"], Extension(client), req).await
}

async fn drive(Extension(client): Extension<Client>, req: Request<Body>) -> Html<String> {
    get_page(&["drive.html"], Extension(client), req).await
}

async fn edit(Extension(client): Extension<Client>, req: Request<Body>) -> Html<String> {
    get_page(&["edit", "[fid].html"], Extension(client), req).await
}

async fn about(Extension(client): Extension<Client>, req: Request<Body>) -> Html<String> {
    get_page(&["about.html"], Extension(client), req).await
}

async fn not_found(Extension(client): Extension<Client>, req: Request<Body>) -> Html<String> {
    get_page(&["404.html"], Extension(client), req).await
}

pub async fn get_page(arr: &[&str], Extension(client): Extension<Client>, req: Request<Body>) -> Html<String> {
    let is_reverse_proxy;
    {
        let cfg = config::inst().read().unwrap();
        is_reverse_proxy = cfg.default.proxy;
    }

    if is_reverse_proxy {
        reverse_proxy::forward_to_nextjs_dev_server(Extension(client), req).await
    } else {
        static_pages::get_page(arr).await
    }
}

pub fn handle_html(html_str: String) -> String {
    let is_reverse_proxy;
    {
        let cfg = config::inst().read().unwrap();
        is_reverse_proxy = cfg.default.proxy;
    }

    let mut script = String::new();

    script.push_str(r#"<script src="/auth/session.js" defer ></script>"#);

    html_str.replace("PAGE_SERVER_PAGE_ARG", &script)
}
