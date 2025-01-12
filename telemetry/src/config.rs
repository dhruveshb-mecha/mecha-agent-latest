use opentelemetry::logs::LogError;
use opentelemetry::{metrics, KeyValue};
use opentelemetry_otlp::{ExportConfig, WithExportConfig};
use opentelemetry_sdk::logs::Config;
use opentelemetry_sdk::{metrics::MeterProvider, runtime, Resource};
use std::time::Duration;
pub fn init_otlp_configuration(exporter_endpoint: &str) -> metrics::Result<MeterProvider> {
    let export_config = ExportConfig {
        endpoint: exporter_endpoint.to_string(),
        ..ExportConfig::default()
    };

    let duration = Duration::from_secs(60); // define duration to export metrics after this duration

    // let exporter = opentelemetry_otlp::new_exporter().
    opentelemetry_otlp::new_pipeline()
        .metrics(runtime::Tokio)
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_export_config(export_config),
        )
        .with_period(duration)
        .with_resource(Resource::new(vec![KeyValue::new(
            opentelemetry_semantic_conventions::resource::SERVICE_NAME,
            "basic-otlp-metrics-example",
        )]))
        .build()
}

pub fn init_logs_config(
    exporter_endpoint: &str,
) -> Result<opentelemetry_sdk::logs::Logger, LogError> {
    opentelemetry_otlp::new_pipeline()
        .logging()
        .with_log_config(Config::default().with_resource(Resource::new(vec![
            KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "mecha-agent-service",
            ),
            KeyValue::new("stream_name", "log_stream"),
        ])))
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(exporter_endpoint),
        )
        .install_batch(runtime::Tokio)
}
