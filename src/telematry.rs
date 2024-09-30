use tracing::Subscriber;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt};

pub fn subscriber<Sink>(name: String, sink: Sink) -> impl Subscriber + Sync + Send
where
    Sink: for<'a> MakeWriter<'a> + 'static + Sync + Send,
{
    use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
    use tracing_subscriber::{registry::Registry, EnvFilter};

    let filter = EnvFilter::try_from_default_env().unwrap_or(
        format!(
            "{}=debug,tower_http=debug,axum::rejection=trace",
            env!("CARGO_CRATE_NAME")
        )
        .into(),
    );

    let formatter = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(filter)
        .with(JsonStorageLayer)
        .with(formatter)
}

pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    tracing_log::LogTracer::init().expect("Unable to bridge logging and tracing");
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set global subscriber");
    tracing::info!("Successfully initialized logging facade for application");
}
