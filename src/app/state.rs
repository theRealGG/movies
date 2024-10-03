use crate::database::Database;

#[derive(Debug, Clone)]
pub struct AppState {
    pub database: Database,
}
