use std::collections::{BTreeMap};
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::{Json, Router, routing::get};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Redirect, Response};




#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { Redirect::permanent("/echo") }))
        .route("/echo", get(echo_request_headers));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[axum_macros::debug_handler]
async fn echo_request_headers(headers: HeaderMap) -> Response {
    // format!("{:?}", headers).into_response()
    let mut header_hashmap = BTreeMap::new();
    for (k, v) in headers {
        let k = k.unwrap().as_str().to_owned();
        let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
        header_hashmap.insert(k, v);
    }
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let start_since_epoch = since_the_epoch.as_secs().to_string();
    header_hashmap.insert("start_since_epoch".to_string(), start_since_epoch);
    header_hashmap.insert("owner".to_string(), "Sri Tikkireddy".to_string());
    header_hashmap.insert("purpose".to_string(), "Echo Headers".to_string());
    header_hashmap.insert("server".to_string(), "axum".to_string());
    header_hashmap.insert("version".to_string(), "0.1.0".to_string());
    header_hashmap.insert("req_source_ip_address".to_string(),
                          header_hashmap.get("x-forwarded-for")
                              .unwrap_or(&"Unknown".to_string())
                              .to_string());

    Json(header_hashmap).into_response()
}