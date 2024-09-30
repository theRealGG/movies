use movies::{
    config::config,
    telematry::{init_subscriber, subscriber},
};

static EXIT_FAILURE: i32 = 1;

fn setup_logger() {
    let subscriber = subscriber("movies".into(), std::io::stdout);
    init_subscriber(subscriber);
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    setup_logger();

    let settings = config().unwrap_or_else(|_| {
        tracing::error!("Unable to load config");
        std::process::exit(EXIT_FAILURE)
    });

    tracing::info!("Successfully loaded config");

    Ok(())
}
