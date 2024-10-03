use std::io::Result as IOResult;

use state::AppState;

use crate::{database::Database, models::settings::Settings, server::Server};

pub mod state;

pub struct Application {
    server: Server,
}

impl Application {
    pub fn try_new(settings: Settings) -> Result<Self, anyhow::Error> {
        let Settings { database, server } = settings;
        let database = Database::new(database);

        Ok(Self {
            server: Server::builder()
                .hostname(server.hostname)
                .port(server.port)
                .reload(server.reload)
                .state(AppState { database })
                .build(),
        })
    }

    pub async fn run(&self) -> IOResult<()> {
        self.server.serve().await
    }
}
