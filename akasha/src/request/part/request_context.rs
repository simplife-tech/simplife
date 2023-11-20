use axum::{async_trait, extract::FromRequestParts};
use http::{request::Parts, StatusCode};
use crate::dto::response::Response;

/// Information extracted from each request
pub struct RequestContext {
    pub user_agent: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for RequestContext
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<Response<String>>);
    // (-500, "failed to get context");
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user_agent = parts
            .headers
            .get(hyper::header::USER_AGENT)
            .and_then(|ua| ua.to_str().ok())
            .unwrap_or("unknown")
            .to_owned();
        Ok(RequestContext { user_agent })
    }
}
