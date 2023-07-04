use std::{net::SocketAddr, collections::HashMap};

use axum::{
    response::{Html, Json},
    routing::get, Router, extract::{ConnectInfo, Query}};


use serde_json::{json, Value};
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .compact()
        .init();

    let app = Router::new()
    .route("/",get(handler))
    .route("/json", get(json_handler))
    ;

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!(addr = ?addr, "connecting");
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

// responses implement IntoResponse
async fn handler(ConnectInfo(peer): ConnectInfo<SocketAddr>) -> Html<&'static str> {
    info!(peer = ?peer, "handling / request");
    Html("<h1>Hello, Internet!</h1>")
}

async fn json_handler(Query(params): Query<HashMap<String, String>>) -> Json<Value> {
    info!("handling request");
    let q: Option<&String> = params.get("q");

    Json(json!({
            "message": "Hello, Internet!",
            "q": q,
        }) )
}