use std::io::Result;

use tokio::net::TcpListener;
use typed_builder::TypedBuilder;

use crate::{app::state::AppState, die, models::environment::Environment, router::router};

#[derive(TypedBuilder, Clone)]
pub struct Server {
    hostname: String,
    port: u16,
    reload: bool,
    state: AppState,
}

impl Server {
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

    pub async fn serve(self) -> Result<()> {
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
        axum::serve(listener, router(self.state))
            .with_graceful_shutdown(shutdown())
            .await
    }
}

fn exit() {
    die!("Terminating process");
}

async fn shutdown() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => exit(),
        _ = terminate => exit(),
    }
}
