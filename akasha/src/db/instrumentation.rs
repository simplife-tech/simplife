
use futures::Future;

#[macro_export]
macro_rules! fetch_one {
    ($ctx:expr, $db:expr, $out_struct:path, $sql:expr, $($item:expr),*) => {{
        let mut query_as = sqlx::query_as::<_, $out_struct>($sql);
        $(
            query_as = query_as.bind($item);
        )*
        let future = query_as.fetch_one($db);
        akasha::db::instrumentation::db_span($ctx, $sql.to_string(), future)
    }};
    
    ($ctx:expr, $db:expr, $sql:expr, $($item:expr),*) => {{
        let mut query = sqlx::query($sql);
        $(
            query = query.bind($item);
        )*
        let future = query.fetch_one($db);
        akasha::db::instrumentation::db_span($ctx, $sql.to_string(), future)
    }}
}

#[macro_export]
macro_rules! fetch_optional {
    ($ctx:expr, $db:expr, $out_struct:path, $sql:expr, $($item:expr),*) => {{
        let mut query_as = sqlx::query_as::<_, $out_struct>($sql);
        $(
            query_as = query_as.bind($item);
        )*
        let future = query_as.fetch_optional($db);
        akasha::db::instrumentation::db_span($ctx, $sql.to_string(), future)
    }};
    
    ($ctx:expr, $db:expr, $sql:expr, $($item:expr),*) => {{
        let mut query = sqlx::query($sql);
        $(
            query = query.bind($item);
        )*
        let future = query.fetch_optional($db);
        akasha::db::instrumentation::db_span($ctx, $sql.to_string(), future)
    }}
}

#[macro_export]
macro_rules! fetch_all {
    ($ctx:expr, $db:expr, $out_struct:path, $sql:expr, $($item:expr),*) => {{
        let mut query_as = sqlx::query_as::<_, $out_struct>($sql);
        $(
            query_as = query_as.bind($item);
        )*
        let future = query_as.fetch_all($db);
        akasha::db::instrumentation::db_span($ctx, $sql.to_string(), future)
    }};
    
    ($ctx:expr, $db:expr, $sql:expr, $($item:expr),*) => {{
        let mut query = sqlx::query($sql);
        $(
            query = query.bind($item);
        )*
        let future = query.fetch_all($db);
        akasha::db::instrumentation::db_span($ctx, $sql.to_string(), future)
    }}
}


#[macro_export]
macro_rules! execute {
    ($ctx:expr, $db:expr, $sql:expr, $($item:expr),*) => {{
        let mut query = sqlx::query($sql);
        $(
            query = query.bind($item);
        )*
        let future = query.execute($db);
        akasha::db::instrumentation::db_span($ctx, $sql.to_string(), future)
    }};
}


pub async fn db_span<O>(ctx: &crate::Context, sql: String, future: impl Future<Output = Result<O, sqlx::Error>> ) -> Result<O, sqlx::Error>
where 
    O: std::fmt::Debug
{
    use std::borrow::Cow;
    use opentelemetry::trace::{TracerProvider, Tracer, Span, Status};
    let tracer = opentelemetry::global::tracer_provider().tracer("");
    let name = if sql.contains("select") {
        "MySQL:Query"
    } else if sql.contains("insert") || sql.contains("update") {
        "MySQL:Exec"
    } else {
        "MySQL:COMMAND"
    };
    let mut span = tracer.start_with_context(name, &ctx.opentelemetry_context);
    span.set_attribute(opentelemetry::Key::new("db.statement").string(sql));
    span.set_attribute(opentelemetry::Key::new("peer.service").string("mysql"));
    let result = future.await;
    if result.is_err() {
        span.set_status(Status::Error { description: Cow::from(result.as_ref().unwrap_err().to_string()) })
    }
    span.end();
    result
}