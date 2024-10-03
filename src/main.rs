use movies::{
    app::Application,
    config::config,
    die,
    telematry::{init_subscriber, subscriber},
};

fn setup_logger() {
    let subscriber = subscriber("movies".into(), std::io::stdout);
    init_subscriber(subscriber);
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    setup_logger();

    let settings = config().unwrap_or_else(|_| die!("Unable to load config"));

    tracing::info!("Successfully loaded config");

    let app =
        Application::try_new(settings).unwrap_or_else(|_| die!("Unable to create application"));

    app.run().await.expect("Could not run application");
    Ok(())
}
