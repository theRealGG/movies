use crate::app::state::AppState;
use axum::Router;

pub fn router(state: AppState) -> Router {
    use crate::{globals::REQUEST_ID, routes::*, telematry::make_span_with};
    use axum::http::HeaderName;
    use axum::routing::get;
    use tower::ServiceBuilder;
    use tower_http::{
        request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
        timeout::TimeoutLayer,
        trace::TraceLayer,
    };

    tracing::info!("Building router");
    let request_id = HeaderName::from_static(REQUEST_ID);

    Router::new()
        .route("/health_check", get(server::health_check))
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::new(request_id.clone(), MakeRequestUuid))
                .layer(TraceLayer::new_for_http().make_span_with(make_span_with))
                .layer(TimeoutLayer::new(core::time::Duration::from_secs(10)))
                .layer(PropagateRequestIdLayer::new(request_id)),
        )
        .with_state(state.clone())
}
