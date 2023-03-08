use axum::{response::Html, routing::get, Router, Extension};
use axum_database_sessions::{AxumSessionConfig, AxumSessionStore, AxumSessionLayer};
use clap::StructOpt;
use hyper::{client::HttpConnector, Body};
use std::net::SocketAddr;

type Client = hyper::client::Client<HttpConnector, Body>;

mod router;
mod storage;
mod utils;
mod args;
mod config;
mod db;
mod session;

#[tokio::main]
async fn main() {
    // args
    let args = args::Args::parse();

    // config
    if let Err(why) = config::load(args.config.as_str(), &args) {
        println!("error occured when loading config.ini.\n{}\nuse default config value", why);
    }

    // init db
    db::DB::connect().await;
    let db = db::DB::inst();
    db.migrate().await;

    // init session
    let session_config = AxumSessionConfig::default()
    .with_table_name("session");

    let session_store = AxumSessionStore::new(Some(db.pool.clone().into()), session_config);
    session_store.migrate().await.unwrap();

    // init server
    let mut app = Router::new();
    app = router::static_files::route(app);
    app = router::auth::route(app);
    app = router::drive::route(app);
    app = router::pages::route(app);
    app = app
        .route("/ping", get(handler_ping))
        .layer(Extension(Client::new()))
        .layer(AxumSessionLayer::new(session_store));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler_ping() -> Html<&'static str> {
    Html("<h1>pong</h1>")
}
