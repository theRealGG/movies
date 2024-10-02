use axum::Router;
use movies::{
    app::state::AppState,
    config::config,
    database::Database,
    models::settings::{DatabaseSettings, Settings},
    server::Server,
};
use sqlx::{Connection, Executor, PgConnection};

pub struct TestApp {
    server: Server,
}

impl TestApp {
    async fn new(settings: Settings) -> Self {
        let maintenance_settings = DatabaseSettings {
            database: "movies".to_string(),
            username: "movies".to_string().into(),
            password: "password".to_string().into(),
            ..settings.database.clone()
        };
        let mut connection =
            PgConnection::connect_with(&maintenance_settings.to_connection_options())
                .await
                .expect("Failed to connect to Postgres");
        connection
            .execute(format!(r#"CREATE DATABASE "{}";"#, settings.database.database).as_str())
            .await
            .expect("Failed to create database.");

        Self {
            server: Server::builder()
                .port(0)
                .hostname("127.0.0.1".into())
                .state(AppState {
                    database: Database::new(settings.database),
                })
                .reload(false)
                .build(),
        }
    }

    fn router(self) -> Router {
        self.server.router()
    }
}

pub async fn spawn() -> Router {
    let settings = {
        let mut initial_config = config().expect("Unable to load config");
        initial_config.database.database = uuid::Uuid::new_v4().to_string();
        initial_config
    };
    let app = TestApp::new(settings).await;
    app.router()
}
