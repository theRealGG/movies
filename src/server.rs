use std::io::Result;

use axum::{http::HeaderName, Router};
use tokio::net::TcpListener;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing_subscriber::fmt::layer;
use typed_builder::TypedBuilder;

use crate::{
    app::state::AppState, models::environment::Environment, routes::server::health_check,
    telematry::make_span_with,
};

#[derive(TypedBuilder)]
pub struct Server {
    hostname: String,
    port: u16,
    reload: bool,
    state: AppState,
}

impl Server {
    pub fn router(&self) -> Router {
        use axum::routing::get;
        use tower::ServiceBuilder;

        let request_id = HeaderName::from_static("x-request-id");

        Router::new()
            .route("/health_check", get(health_check))
            .layer(
                ServiceBuilder::new()
                    .layer(SetRequestIdLayer::new(request_id.clone(), MakeRequestUuid))
                    .layer(TraceLayer::new_for_http().make_span_with(make_span_with))
                    .layer(PropagateRequestIdLayer::new(request_id)),
            )
            .with_state(self.state.clone())
    }

    async fn without_reload(&self) -> Result<TcpListener> {
        tracing::info!("Spawning server without reload functionality");
        TcpListener::bind(format!("{}:{}", self.hostname, self.port)).await
    }

    async fn with_reload(&self) -> Result<TcpListener> {
        use listenfd::ListenFd;
        tracing::info!("Spawning server with reload functionality");
        let curr_env = Environment::current_env();
        if curr_env != Environment::Local {
            tracing::warn!(
                %curr_env,
                "The reloading functionality should only be used for local development"
            );
        }
        let mut listenfd = ListenFd::from_env();
        match listenfd.take_tcp_listener(0).unwrap() {
            Some(listener) => {
                listener.set_nonblocking(true).unwrap();
                Ok(TcpListener::from_std(listener)?)
            }
            None => Ok(TcpListener::bind(format!("{}:{}", self.hostname, self.port)).await?),
        }
    }

    pub async fn serve(&self) -> Result<()> {
        let listener = if self.reload {
            self.with_reload().await?
        } else {
            self.without_reload().await?
        };
        tracing::info!(
            hostname = self.hostname,
            port = self.port,
            "Spawning server"
        );
        axum::serve(listener, self.router()).await
    }
}
