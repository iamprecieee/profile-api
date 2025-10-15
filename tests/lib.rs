use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::get,
    Router,
};
use hng_stage_0::{api::AppContext, config::GlobalConfig, routes::fetch_profile, DEFAULT_CAT_FACT};
use serde_json::{from_slice, json, Value};
use tower::ServiceExt;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

fn create_test_config(test_cat_api_url: String) -> GlobalConfig {
    GlobalConfig {
        host: String::from("localhost"),
        port: 8000,
        cors_allowed_origins: vec![String::from("http://localhost")],
        email: String::from("test@example.com"),
        full_name: String::from("Test User"),
        stack: String::from("Rust/Axum"),
        cat_fact_api: test_cat_api_url,
    }
}

fn create_test_context(config: GlobalConfig) -> AppContext {
    AppContext { config }
}

#[tokio::test]
async fn test_fetch_profile_success() {
    let mock_server = MockServer::start().await;
    let test_cat_fact = "Cats sleep 70% of their lives.";

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "fact": test_cat_fact
        })))
        .mount(&mock_server)
        .await;

    let config = create_test_config(mock_server.uri());
    let context = create_test_context(config);

    let app = Router::new()
        .route("/me", get(fetch_profile))
        .with_state(context);

    let response = app
        .oneshot(Request::builder().uri("/me").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = from_slice(&body).unwrap();

    assert_eq!(json["status"], "success");
    assert_eq!(json["user"]["email"], "test@example.com");
    assert_eq!(json["user"]["name"], "Test User");
    assert_eq!(json["user"]["stack"], "Rust/Axum");
    assert_eq!(json["fact"], test_cat_fact);
    assert!(json["timestamp"].is_string());
}

#[tokio::test]
async fn test_fetch_profile_with_api_failure() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let config = create_test_config(mock_server.uri());
    let context = create_test_context(config);

    let app = Router::new()
        .route("/me", get(fetch_profile))
        .with_state(context);

    let response = app
        .oneshot(Request::builder().uri("/me").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = from_slice(&body).unwrap();

    assert_eq!(json["status"], "success");
    assert_eq!(json["fact"], DEFAULT_CAT_FACT);
}
