use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::error_handling::HandleErrorLayer;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::{routing::get, BoxError, Json, Router};
use lru_time_cache::LruCache;
use reqwest;
use reqwest::Client;
use serde_json::Value;
use tokio::sync::Mutex;
use tower::buffer::BufferLayer;
use tower::limit::RateLimitLayer;
use tower::ServiceBuilder;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info, Span};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Clone)]
struct AppState {
    // We require unique usernames. This tracks which usernames have been taken.
    ip_lookup_cache: Arc<Mutex<LruCache<String, String>>>,
    client: Client,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "echo_server=info,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Construct an `LruCache` of `<String, String>`s, limited by key count
    let max_count = 500;
    let time_to_live = Duration::from_secs(60);
    let lru_time_cache = Mutex::new(
        LruCache::<String, String>::with_expiry_duration_and_capacity(time_to_live, max_count),
    );
    let app_state = AppState {
        ip_lookup_cache: Arc::new(lru_time_cache),
        client: Client::new(),
    };

    let app = Router::new()
        .route("/", get(|| async { Redirect::permanent("/echo/headers") }))
        .route("/healthz", get(health))
        .route("/echo/ip", get(echo_ip))
        .route("/echo/ip/:some_id", get(get_ip_details))
        .route("/echo/ip-details/:some_id", get(get_ip_details))
        .route("/echo/cache-size", get(cache_size))
        .route("/echo/github-url", get(github_url))
        .route("/echo/headers", get(echo_request_headers))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled error: {}", err),
                    )
                }))
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(100, Duration::from_secs(1))),
        )
        .layer(
            TraceLayer::new_for_http()
                .on_request(())
                .on_response(())
                .on_body_chunk(())
                .on_eos(())
                .on_failure(|_: ServerErrorsFailureClass, _: Duration, _: &Span| {
                    tracing::debug!("something went wrong")
                }),
        )
        .with_state(app_state);

    println!("Starting web server!");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn fetch_header(headers: HeaderMap, header_name: &str) -> String {
    match headers.get(header_name.to_string()) {
        None => "Unknown".to_string(),
        Some(value) => {
            // "Unknown".into_response()
            String::from_utf8_lossy(value.as_bytes()).into_owned()
        }
    }
}

#[axum_macros::debug_handler]
async fn health() -> Response {
    return "OK".clone().into_response();
}

#[axum_macros::debug_handler]
async fn cache_size(State(state): State<AppState>) -> Response {
    let cache = state.ip_lookup_cache.lock().await;
    return cache.len().to_string().into_response();
}

#[axum_macros::debug_handler]
async fn get_ip_details(State(state): State<AppState>, Path(some_id): Path<String>) -> Response {
    let mut cache = state.ip_lookup_cache.lock().await;

    return if cache.contains_key(&some_id) {
        cache.get(&some_id).unwrap().to_owned().into_response()
    } else {
        info!(
            "Cache miss for {}.xxx.xxx.xxx",
            some_id.split(".").collect::<Vec<&str>>()[0],
        );
        let ip_details = fetch_ip_details(state.client.clone(), some_id.clone()).await;
        cache.insert(some_id.clone(), ip_details.clone());
        ip_details.into_response()
    };
}

#[axum_macros::debug_handler]
async fn echo_ip(headers: HeaderMap) -> Response {
    fetch_header(headers, "x-forwarded-for").into_response()
}

#[axum_macros::debug_handler]
async fn github_url() -> Response {
    "https://github.com/tsriharsha/echo-server".into_response()
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
    header_hashmap.insert(
        "req_source_ip_address".to_string(),
        header_hashmap
            .get("x-forwarded-for")
            .unwrap_or(&"Unknown".to_string())
            .to_string(),
    );

    Json(header_hashmap).into_response()
}

pub async fn fetch_ip_details(client: Client, ip_address: String) -> String {
    let req_url = format!(
        "http://ip-api.com/json/{ip_address}?fields=status,message,continent,\
        continentCode,country,countryCode,region,regionName,city,\
        district,zip,lat,lon,timezone,offset,currency,isp,org,as,\
        asname,reverse,mobile,proxy,hosting,query",
        ip_address = ip_address
    );
    let resp = client
        .get(req_url.clone())
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let mut deserialized_json: BTreeMap<String, Value> = serde_json::from_str(&resp).unwrap();
    deserialized_json.insert("request_url".to_string(), req_url.into());
    serde_json::to_string(&deserialized_json).unwrap()
}
