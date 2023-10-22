use async_trait::async_trait;
use eyre::Context;
use hyper::body::Body;
use kitsune_config::Configuration;
use metrics_opentelemetry::OpenTelemetryRecorder;
use metrics_tracing_context::{MetricsLayer, TracingContextLayer};
use metrics_util::layers::Layer as _;
use opentelemetry::{
    metrics::{noop::NoopMeterProvider, Meter, MeterProvider},
    runtime::Tokio,
    trace::{noop::NoopTracer, Tracer},
};
use opentelemetry_http::{Bytes, HttpClient, HttpError, Request, Response};
use opentelemetry_otlp::WithExportConfig;
use std::{env, fmt};
use tracing_error::ErrorLayer;
use tracing_opentelemetry::{OpenTelemetryLayer, PreSampledTracer};
use tracing_subscriber::{
    filter::{LevelFilter, Targets},
    layer::SubscriberExt,
    Layer as _, Registry,
};

#[derive(Clone)]
struct HttpClientAdapter {
    inner: kitsune_http_client::Client,
}

impl fmt::Debug for HttpClientAdapter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HttpClientAdapter")
    }
}

#[async_trait]
impl HttpClient for HttpClientAdapter {
    async fn send(&self, request: Request<Vec<u8>>) -> Result<Response<Bytes>, HttpError> {
        let (parts, body) = request.into_parts();
        let body = Body::from(body);
        let request = Request::from_parts(parts, body);

        let response = self.inner.execute(request).await?.into_inner();

        let (parts, body) = response.into_parts();
        let body = hyper::body::to_bytes(body).await?;

        Ok(Response::from_parts(parts, body))
    }
}

fn initialise_logging<T>(tracer: T) -> eyre::Result<()>
where
    T: Tracer + PreSampledTracer + Send + Sync + 'static,
{
    let env_filter = env::var("RUST_LOG")
        .map_err(eyre::Report::from)
        .and_then(|targets| targets.parse().context("Failed to parse RUST_LOG value"))
        .unwrap_or_else(|_| Targets::default().with_default(LevelFilter::INFO));

    let subscriber = Registry::default()
        .with(tracing_subscriber::fmt::layer().with_filter(env_filter))
        .with(ErrorLayer::default())
        .with(OpenTelemetryLayer::new(tracer));

    let subscriber = subscriber.with(MetricsLayer::new());

    tracing::subscriber::set_global_default(subscriber)
        .context("Couldn't install the global tracing subscriber")?;

    Ok(())
}

fn initialise_metrics(meter: Meter) -> eyre::Result<()> {
    let recorder = TracingContextLayer::all().layer(OpenTelemetryRecorder::new(meter));
    metrics::set_boxed_recorder(Box::new(recorder))
        .context("Couldn't install the global metrics recorder")?;

    Ok(())
}

pub fn initialise(app_name: &'static str, config: &Configuration) -> eyre::Result<()> {
    if let Some(ref opentelemetry_config) = config.opentelemetry {
        let http_client = HttpClientAdapter {
            inner: kitsune_http_client::Client::default(),
        };

        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .http()
                    .with_endpoint(opentelemetry_config.http_endpoint.as_str())
                    .with_http_client(http_client.clone()),
            )
            .install_batch(Tokio)?;

        initialise_logging(tracer)?;

        let meter_provider = opentelemetry_otlp::new_pipeline()
            .metrics(Tokio)
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .http()
                    .with_endpoint(opentelemetry_config.http_endpoint.as_str())
                    .with_http_client(http_client.clone()),
            )
            .build()?;

        initialise_metrics(meter_provider.meter(app_name))?;
    } else {
        initialise_logging(NoopTracer::new())?;
        initialise_metrics(NoopMeterProvider::new().meter(app_name))?;
    }

    Ok(())
}