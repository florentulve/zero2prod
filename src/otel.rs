//! Instrumentation OpenTelemetry - Métriques et traces distribuées.
//!
//! Configure l'export OTLP vers un collecteur OpenTelemetry avec fallback stdout.

use opentelemetry::{KeyValue, global, trace::TracerProvider};
use opentelemetry_sdk::{
    Resource,
    metrics::{MeterProviderBuilder, PeriodicReader, SdkMeterProvider},
    trace::{RandomIdGenerator, Sampler, SdkTracerProvider, Tracer},
};
use opentelemetry_semantic_conventions::attribute::{
    DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_NAME, SERVICE_VERSION,
};
use std::env;
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

/// Ressource OTel identifiant ce service (nom, version, environnement).
fn resource() -> Resource {
    Resource::builder()
        .with_attributes([
            KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
            KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, "develop"),
        ])
        .build()
}

fn init_meter_provider() -> SdkMeterProvider {
    let mut builder = MeterProviderBuilder::default().with_resource(resource());

    let exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .build()
        .unwrap();

    let reader = PeriodicReader::builder(exporter)
        .with_interval(std::time::Duration::from_secs(30))
        .build();

    builder = builder.with_reader(reader);

    let meter_provider = builder.build();

    global::set_meter_provider(meter_provider.clone());
    meter_provider
}

/// Initialise le TracerProvider avec sampling 100%.
fn init_tracer() -> Tracer {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .unwrap();

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
            1.0, // 100% sampling
        ))))
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(resource())
        .build();

    global::set_tracer_provider(provider.clone());
    provider.tracer("tracing-otel-subscriber")
}

/// Crée un compteur de test.
fn init_metrics() {
    let meter = global::meter("my_service");
    let counter = meter.u64_counter("my_counter").build();
    counter.add(1, &[KeyValue::new("http.client_ip", "83.164.160.102")]);
}

/// Initialise tracing-subscriber avec OpenTelemetry.
/// Retourne OtelGuard pour le shutdown propre à la terminaison.
pub fn init_tracing_subscriber() -> OtelGuard {
    let meter_provider = init_meter_provider();
    let tracer = init_tracer();

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .with_timer(tracing_subscriber::fmt::time::SystemTime)
        .with_level(true)
        .with_file(true)
        .with_line_number(true);

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(MetricsLayer::new(meter_provider.clone()))
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    init_metrics();
    OtelGuard { meter_provider }
}

/// Guard assurant le shutdown propre du MeterProvider.
pub struct OtelGuard {
    meter_provider: SdkMeterProvider,
}

impl Drop for OtelGuard {
    fn drop(&mut self) {
        if let Err(err) = self.meter_provider.shutdown() {
            eprintln!("{err:?}");
        }
    }
}
