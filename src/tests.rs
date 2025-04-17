use axum_test::TestServer;
use serde_json::json;

async fn create_test_app() -> TestServer {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("no DATABASE_URL");

    let router = crate::create_router(&database_url)
        .await
        .expect("cannot create router");

    TestServer::builder()
        .expect_success_by_default()
        .http_transport()
        .build(router)
        .expect("cannot build test server")
}

#[tokio::test(flavor = "multi_thread")]
async fn health() {
    let app = create_test_app().await;

    let resp = app.get("/api/health").await;

    resp.assert_status_ok();
}

#[tokio::test(flavor = "multi_thread")]
async fn create_subject() {
    let app = create_test_app().await;

    let name = "a test subject";

    let create_resp = app.post("/api/subjects").json(&json!({"name": name})).await;

    let body = create_resp.json::<serde_json::Value>();
    let id = body
        .get("id")
        .expect("no id in result")
        .as_u64()
        .expect("id is not an int");

    let resp = app.get(&format!("/api/subjects/{id}")).await;

    resp.assert_json_contains(&json!({"name": name}));
}
