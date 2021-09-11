use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn register_subscriber(name: &str, directives: &str) -> impl Subscriber + Send + Sync {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(directives));
    let formatting_layer = BunyanFormattingLayer::new(name.into(), std::io::stdout);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_tracing(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set up log tracer.");
    set_global_default(subscriber).expect("Failed to set global default subscriber.");
}
