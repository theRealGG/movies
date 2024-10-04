use axum::http::Request;
use tracing::{error, info_span, Span, Subscriber};
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt};

use crate::globals::REQUEST_ID;

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

pub fn make_span_with<T>(request: &Request<T>) -> Span {
    use tracing::{error, info_span};
    use uuid::Uuid;
    match request.headers().get(REQUEST_ID) {
        Some(id) => {
            info_span!(
                "HTTP-REQUEST",
                correlation_id = id
                    .to_str()
                    .map(String::from)
                    .unwrap_or_else(|_| Uuid::new_v4().to_string())
            )
        }
        None => {
            error!("Unable to extract request_id. Make sure the request_id is set");
            info_span!("HTTP-REQUEST", correlation_id = %Uuid::new_v4())
        }
    }
}
