use std::io::Result as IOResult;

use anyhow::Ok;

use crate::{models::settings::Settings, server::Server};

pub mod state;

pub struct Application {
    server: Server,
}

impl Application {
    pub fn try_new(settings: Settings) -> Result<Self, anyhow::Error> {
        Ok(Self {
            server: Server::builder()
                .hostname(settings.server.hostname)
                .port(settings.server.port)
                .reload(settings.server.reload)
                .build(),
        })
    }

    pub async fn run(&self) -> IOResult<()> {
        self.server.serve().await
    }
}
