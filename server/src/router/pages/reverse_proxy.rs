use axum::{
    extract::Extension,
    http::{uri::Uri, Request, Response, StatusCode},
    response::Html,
    routing::get,
    Router,
};
use hyper::{client::HttpConnector, Body};
use std::convert::TryFrom;

type Client = hyper::client::Client<HttpConnector, Body>;

pub fn route(app: Router<Body>) -> Router<Body> {
    app.fallback(get(handler_nextjs_dev))
}

async fn handler_nextjs_dev(Extension(client): Extension<Client>, mut req: Request<Body>) -> Response<Body> {
    let path = req.uri().path();
    let path_query = req.uri().path_and_query().map(|v| v.as_str()).unwrap_or(path);

    let mut uri_base = String::new();
    let rusty = path_query.find("/rusty");
    if let Some(i) = rusty {
        if 0 == i {
            uri_base.push_str("http://127.0.0.1:3048");
        }
    }

    if uri_base.is_empty() {
        return Response::new(Body::from(format!("unknown request: {}", path_query)));
    }

    let uri = format!("{}{}", uri_base, path_query);
    *req.uri_mut() = Uri::try_from(uri).unwrap();
    match client.request(req).await {
        Ok(resp) => {
            return resp;
        }
        Err(why) => {
            return Response::new(Body::from(format!("request error: {}", why)));
        }
    }
}

pub async fn forward_to_nextjs_dev_server(Extension(client): Extension<Client>, req: Request<Body>) -> Html<String> {
    let mut resp = handler_nextjs_dev(Extension(client), req).await;

    if resp.status() == StatusCode::OK {
        let body = resp.body_mut();
        let bytes = hyper::body::to_bytes(body).await.unwrap();
        let result = String::from_utf8(bytes.into_iter().collect()).expect("");
        return Html(crate::router::pages::handle_html(result));
    }

    return Html(String::from("error!"));
}
