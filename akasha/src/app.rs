
use axum::{middleware::Next, response::IntoResponse, extract::State};
use http::{Request, StatusCode, HeaderValue};
use hyper::{body::{to_bytes, Bytes}, Body};
use serde_json::json;
use tokio::signal;
use opentelemetry::{trace::{TraceError, Tracer, TraceContextExt, TracerProvider, TraceId}, sdk::trace::{self as sdktrace}, Key, global};

#[macro_export]
macro_rules! instrumented_redis_cmd {
    ($oc:expr, $conn:expr, $key:expr, $($arg:tt)*) => {{
        let tracer = akasha::opentelemetry::global::tracer_provider().tracer("");
        let token = stringify!($($arg)*);
        let name = if token.contains("get") {
            "Redis:GET"
        } else if token.contains("set") {
            "Redis:SET"
        } else {
            "Redis:COMMAND"
        };
        let mut span = tracer.start_with_context(name, &$oc);
        span.set_attribute(akasha::opentelemetry::Key::new("db.key").string($key.to_string()));
        span.set_attribute(akasha::opentelemetry::Key::new("db.statement").string(stringify!($($arg)*)));
        span.set_attribute(akasha::opentelemetry::Key::new("peer.service").string("redis"));
        let result = $conn.$($arg)*.await;
        span.end();
        result
    }};
}

pub async fn trace_http<B>(
    mut req: Request<B>,
    next: Next<B>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    opentelemetry::global::tracer_provider().tracer("").in_span(req.uri().path().to_string(), |cx| async move {
        let span = cx.span();
        span.set_attribute(Key::new("http.method").string(req.method().to_string()));
        span.set_attribute(Key::new("http.target").string(req.uri().to_string()));
        let trace_id = span.span_context().trace_id();
        req.extensions_mut().insert(cx.clone());

        let res = next.run(req).await;
        let (parts, body) = res.into_parts();
        let bytes = to_bytes(body).await.unwrap_or(Bytes::default());
        let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap_or(json!({ "code": -500 }));
        span.set_attribute(Key::new("http.ecode").i64(v.get("code").unwrap_or(&json!(-500)).as_i64().unwrap_or(-500)));
        let mut res = http::Response::from_parts(parts, Body::from(bytes));
        span.set_attribute(Key::new("http.status_code").i64(res.status().as_u16() as i64));
        match res.headers_mut().try_entry("simplife-trace-id") {
            Ok(entry) => {
                match entry {
                    axum::http::header::Entry::Occupied(mut _val) => {
                        //has val
                    }
                    axum::http::header::Entry::Vacant(val) => {
                        val.insert(HeaderValue::from_str(&trace_id.to_string()).unwrap());
                    }
                }
            },
            Err(_) => {
                res.headers_mut().append("simplife-trace-id", HeaderValue::from_str(&trace_id.to_string()).unwrap());
            },
        };
        Ok(res)
    }).await
}

pub fn init_tracer(endpoint: String, service_name: String, service_version: String) {
    let provider = opentelemetry_jaeger::new_agent_pipeline()
        .with_endpoint(endpoint)
        .with_service_name(&service_name)
        .with_trace_config(opentelemetry::sdk::trace::config().with_resource(
            opentelemetry::sdk::Resource::new(vec![
                opentelemetry::KeyValue::new("service.name", service_name), 
                opentelemetry::KeyValue::new("service.version", service_version)
            ]),
        ))
        .build_batch(opentelemetry::runtime::Tokio);
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    opentelemetry::global::set_tracer_provider(provider.unwrap());
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