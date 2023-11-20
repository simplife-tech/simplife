
use opentelemetry::global;
use tokio::signal;

#[macro_export]
macro_rules! instrumented_redis_cmd {
    ($oc:expr, $conn:expr, $key:expr, $($arg:tt)*) => {{
        let tracer = akasha::opentelemetry::global::tracer_provider().tracer("");
        let token = stringify!($($arg)*);
        let name = if token.contains("hget") {
            "Redis:HGET"
        } else if token.contains("hset") {
            "Redis:HSET"
        } else if token.contains("get") {
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

#[macro_export]
macro_rules! instrumented_mysql_cmd {
    ($oc:expr, $key:expr, $($arg:tt)*) => {{
        let tracer = akasha::opentelemetry::global::tracer_provider().tracer("");
        let name = if $key.contains("select") {
            "MySQL:Query"
        } else if $key.contains("insert") || $key.contains("update") {
            "MySQL:Exec"
        } else {
            "MySQL:COMMAND"
        };
        let mut span = tracer.start_with_context(name, &$oc);
        span.set_attribute(akasha::opentelemetry::Key::new("db.statement").string($key));
        span.set_attribute(akasha::opentelemetry::Key::new("peer.service").string("mysql"));
        let result = $($arg)*.await;
        span.end();
        result
    }};
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