use std::{task::{Poll, Context, ready}, pin::Pin, future::Future};

use http::{Request, Response, HeaderValue};
use tokio::signal;
use opentelemetry::{trace::{TraceError, Tracer, TraceContextExt}, sdk::trace as sdktrace, Key};
use tower::{Layer, Service};
use pin_project_lite::pin_project;

#[derive(Clone)]
pub struct TraceLayer {
    pub tracer: sdktrace::Tracer,
}

impl<S> Layer<S> for TraceLayer {
    type Service = TraceService<S>;

    fn layer(&self, service: S) -> Self::Service {
        TraceService {
            tracer: self.tracer.clone(),
            service
        }
    }
}

#[derive(Clone)]
pub struct TraceService<S> {
    tracer: sdktrace::Tracer,
    service: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for TraceService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request<ReqBody>) -> Self::Future {
        self.tracer.in_span(request.uri().to_string(), |cx| {
            let span = cx.span();
            span.set_attribute(Key::new("http.method").string(request.method().to_string()));
            span.set_attribute(Key::new("http.target").string(request.uri().to_string()));
            ResponseFuture {
                inner: self.service.call(request),
                trace_id: span.span_context().trace_id().to_string()
            }
        })
    }
}

pin_project! {
    pub struct ResponseFuture<F> {
        #[pin]
        inner: F,
        trace_id: String,
    }
}

impl<F, ResBody, E> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response<ResBody>, E>>,
{
    type Output = Result<Response<ResBody>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let trace_id = self.trace_id.clone();
        let mut response: Response<ResBody> = ready!(self.project().inner.poll(cx))?;
        let hdr = response.headers_mut();
        match hdr.try_entry("simplife-trace-id") {
            Ok(entry) => {
                match entry {
                    axum::http::header::Entry::Occupied(mut val) => {
                        //has val
                    }
                    axum::http::header::Entry::Vacant(val) => {
                        val.insert(HeaderValue::from_str(&trace_id).unwrap());
                    }
                }
            }
            Err(_) => {
                hdr.append(
                    "simplife-trace-id",
                    HeaderValue::from_str(&trace_id).unwrap(),
                );
            }
        }
        Poll::Ready(Ok(response))
    }
}

pub fn init_tracer(endpoint: String, service_name: String, service_version: String) -> Result<sdktrace::Tracer, TraceError> {
    opentelemetry::global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    opentelemetry_jaeger::new_agent_pipeline()
        .with_endpoint(endpoint)
        .with_service_name(&service_name)
        .with_trace_config(opentelemetry::sdk::trace::config().with_resource(
            opentelemetry::sdk::Resource::new(vec![
                opentelemetry::KeyValue::new("service.name", service_name), // this will not override the trace-udp-demo
                opentelemetry::KeyValue::new("service.version", service_version)
            ]),
        ))
        .install_batch(opentelemetry::runtime::Tokio)
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
  
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
  
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
  
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
  
    println!("signal received, starting graceful shutdown");
}