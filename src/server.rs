use std::io::Result;

use axum::Router;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use typed_builder::TypedBuilder;

use crate::{models::environment::Environment, telematry::make_span_with};

#[derive(Debug, TypedBuilder)]
pub struct Server {
    hostname: String,
    port: u16,
    reload: bool,
}

impl Server {
    fn router(&self) -> Router {
        use tower::ServiceBuilder;
        Router::new().layer(
            ServiceBuilder::new().layer(TraceLayer::new_for_http().make_span_with(make_span_with)),
        )
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
