use crate::helper::spawn;
use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn should_pass_health_check() {
    let app = spawn().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health_check")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}
