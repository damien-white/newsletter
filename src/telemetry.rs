use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

pub fn register_subscriber<W>(
    name: &str,
    directives: &str,
    writer: W,
) -> impl Subscriber + Send + Sync
where
    W: MakeWriter + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(directives));
    let formatting_layer = BunyanFormattingLayer::new(name.into(), writer);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber<S>(subscriber: S)
where
    S: Subscriber + Send + Sync,
{
    LogTracer::init().expect("Failed to set up log tracer.");
    set_global_default(subscriber).expect("Failed to set global default subscriber.");
}
