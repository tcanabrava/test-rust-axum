use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing::Subscriber;
use tracing_log::LogTracer;

pub fn log_subscriber(name: String, env_filter: String)
    -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(
        name,
        std::io::stdout);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_logging(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init()
        .expect("Failed to set logger");

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default");
}
