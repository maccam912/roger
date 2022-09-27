use tracing_subscriber::prelude::*;
use eyre::Result;

pub(crate) fn init_tracing() -> Result<()> {
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("roger")
        .install_simple()?;

    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    let subscriber = tracing_subscriber::Registry::default().with(telemetry_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting the default subscriber failed");
    Ok(())
}
