use futures::Future;
use redis::RedisError;


#[macro_export]
macro_rules! get {
    ($ctx:expr, $manager:expr, $key:expr) => {{
        let future = $manager.get($key);
        akasha::redis::instrumentation::redis_span($ctx, format!("GET {}", $key), future)
    }};
}

#[macro_export]
macro_rules! hget {
    ($ctx:expr, $manager:expr, $key:expr, $field:expr) => {{
        let future = $manager.hget($key, $field);
        akasha::redis::instrumentation::redis_span($ctx, format!("HGET {} {}", $key, $field), future)
    }};
}

#[macro_export]
macro_rules! set_ex {
    ($ctx:expr, $manager:expr, $key:expr, $value:expr, $seconds:expr) => {{
        let future = $manager.set_ex::<_, _, ()>($key, $value, $seconds as usize);
        akasha::redis::instrumentation::redis_span($ctx, format!("SETEX {} {} {}", $key, $seconds, $value), future)
    }};
}

#[macro_export]
macro_rules! hset {
    ($ctx:expr, $manager:expr, $key:expr, $field:expr, $value:expr) => {{
        let future = $manager.hset::<_, _, _, ()>($key, $field, $value);
        akasha::redis::instrumentation::redis_span($ctx, format!("HSET {} {} {}", $key, $field, $value), future)
    }};
}

#[macro_export]
macro_rules! del {
    ($ctx:expr, $manager:expr, $key:expr) => {{
        let future = $manager.del::<_, ()>($key);
        akasha::redis::instrumentation::redis_span($ctx, format!("DEL {}", $key), future)
    }};
}

#[macro_export]
macro_rules! hexists {
    ($ctx:expr, $manager:expr, $key:expr, $field:expr) => {{
        let future = $manager.hexists($key, $field);
        akasha::redis::instrumentation::redis_span($ctx, format!("HEXISTS {} {}", $key, $field), future)
    }};
}

#[macro_export]
macro_rules! expire {
    ($ctx:expr, $manager:expr, $key:expr, $seconds:expr) => {{
        let future = $manager.expire::<_, ()>($key, $seconds as usize);
        akasha::redis::instrumentation::redis_span($ctx, format!("EXPIRE {} {}", $key, $seconds), future)
    }};
}

#[macro_export]
macro_rules! ttl {
    ($ctx:expr, $manager:expr, $key:expr) => {{
        let future = $manager.ttl($key);
        akasha::redis::instrumentation::redis_span($ctx, format!("TTL {}", $key), future)
    }};
}

pub async fn redis_span<O>(ctx: &crate::Context, command: String, future: impl Future<Output = Result<O, RedisError>> ) -> Result<O, RedisError>
where 
    O: std::fmt::Debug
{
    use std::borrow::Cow;
    use opentelemetry::trace::{TracerProvider, Tracer, Span, Status};
    let tracer = opentelemetry::global::tracer_provider().tracer("");
    let name = if command.contains("HGET") {
        "Redis:HGET"
    } else if command.contains("HSET") {
        "Redis:HSET"
    } else if command.contains("GET") {
        "Redis:GET"
    } else if command.contains("SET") {
        "Redis:SET"
    } else {
        "Redis:COMMAND"
    };
    let mut span = tracer.start_with_context(name, &ctx.opentelemetry_context);
    span.set_attribute(opentelemetry::Key::new("db.statement").string(command));
    span.set_attribute(opentelemetry::Key::new("peer.service").string("redis"));
    let result = future.await;
    if result.is_err() {
        span.set_status(Status::Error { description: Cow::from(result.as_ref().unwrap_err().to_string()) })
    }
    span.end();
    result
}