use std::{str::FromStr, pin::Pin, task::{Poll, Context}};

use hyper::Body;
use opentelemetry::{global, propagation::{Injector, Extractor}, trace::{TracerProvider, Tracer, Span, TraceContextExt}};
use tonic::{body::BoxBody, Request, Status, metadata::{MetadataMap, MetadataKey, KeyRef}};
use tower::{Layer, Service};

#[derive(Debug, Clone, Default)]
pub struct GrpcOpentelemetryLayer;

impl<S> Layer<S> for GrpcOpentelemetryLayer {
    type Service = GrpcOpentelemetry<S>;

    fn layer(&self, service: S) -> Self::Service {
        GrpcOpentelemetry { inner: service }
    }
}

#[derive(Debug, Clone)]
pub struct GrpcOpentelemetry<S> {
    inner: S,
}

type BoxFuture<'a, T> = Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

impl<S> Service<hyper::Request<Body>> for GrpcOpentelemetry<S>
where
    S: Service<hyper::Request<Body>, Response = hyper::Response<BoxBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: hyper::Request<Body>) -> Self::Future {
        // This is necessary because tonic internally uses `tower::buffer::Buffer`.
        // See https://github.com/tower-rs/tower/issues/547#issuecomment-767629149
        // for details on why this is necessary
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let uri = req.uri().clone();
        let method = req.method().clone();
        let mut tonic_req = tonic::Request::from_http(req);
        
        let oc = global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataExtractor(tonic_req.metadata()))
        });

        Box::pin(async move {
            let tracer = opentelemetry::global::tracer_provider().tracer("");
            let span = tracer.start_with_context(uri.path().to_string(), &oc);
            let oc = opentelemetry::Context::current_with_span(span);
            let (metadata, extensions, message) = tonic_req.into_parts();
            let mut req = hyper::Request::builder()
            .method(method)
            .extension(oc)
            .uri(&uri)
            .body(message)
            .unwrap();
            *req.headers_mut() = metadata.into_headers();

            let response = inner.call(req).await?;
            Ok(response)
        })
    }
}

pub struct MetadataInjector<'a>(pub &'a mut MetadataMap);

impl<'a> Injector for MetadataInjector<'a> {
    /// Set a key and value in the MetadataMap.  Does nothing if the key or value are not valid inputs
    fn set(&mut self, key: &str, value: String) {
        if let Ok(key) = tonic::metadata::MetadataKey::from_bytes(key.as_bytes()) {
            if let Ok(val) = tonic::metadata::MetadataValue::try_from(&value) {
                self.0.insert(key, val);
            }
        }
    }
}


pub struct MetadataExtractor<'a>(&'a MetadataMap);

impl<'a> Extractor for MetadataExtractor<'a> {
    /// Get a value for a key from the MetadataMap.  If the value can't be converted to &str, returns None
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|metadata| metadata.to_str().ok())
    }

    /// Collect all the keys from the MetadataMap.
    fn keys(&self) -> Vec<&str> {
        self.0
            .keys()
            .map(|key| match key {
                KeyRef::Ascii(v) => v.as_str(),
                KeyRef::Binary(v) => v.as_str(),
            })
            .collect::<Vec<_>>()
    }
}

// pub fn grpc_intercept(mut req: Request<()>) -> Result<Request<()>, Status> {
//     let oc = global::get_text_map_propagator(|propagator| {
//         propagator.extract(&MetadataExtractor(req.metadata()))
//     });
//     req.extensions_mut().insert(oc);
//     Ok(req)
// }