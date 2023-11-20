use std::borrow::Cow;

use axum::{middleware::Next, response::IntoResponse};
use http::{Request, StatusCode, HeaderValue};
use opentelemetry::{trace::{TracerProvider, Tracer, TraceContextExt, Status}, Key};

use crate::Context;

pub async fn trace_http<B>(
    mut req: Request<B>,
    next: Next<B>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    opentelemetry::global::tracer_provider().tracer("").in_span(req.uri().path().to_string(), |cx| async move {
        let span = cx.span();
        span.set_attribute(Key::new("http.method").string(req.method().to_string()));
        span.set_attribute(Key::new("http.target").string(req.uri().to_string()));
        let trace_id = span.span_context().trace_id();
        let ctx = req.extensions_mut().get_mut::<Context>().unwrap();
        ctx.opentelemetry_context = cx.clone();
        let mut res = next.run(req).await;
        let status_code = res.status().as_u16();
        if status_code >= 400 {
            span.set_status(Status::Error { description: Cow::from("ERROR") })
        }
        span.set_attribute(Key::new("http.status_code").i64(status_code as i64));
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