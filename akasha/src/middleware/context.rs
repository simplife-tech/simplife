use std::task::{Poll, Context};
extern crate tokio;
use hyper::Body;
use tower::{Layer, Service};

#[derive(Debug, Clone, Default)]
pub struct ContextLayer;

impl<S> Layer<S> for ContextLayer {
    type Service = ContextService<S>;

    fn layer(&self, service: S) -> Self::Service {
        ContextService { inner: service }
    }
}

#[derive(Debug, Clone)]
pub struct ContextService<S> {
    inner: S,
}

impl<S> Service<hyper::Request<Body>> for ContextService<S>
where
    S: Service<hyper::Request<Body>> + Clone
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: hyper::Request<Body>) -> Self::Future {
        // This is necessary because tonic internally uses `tower::buffer::Buffer`.
        // See https://github.com/tower-rs/tower/issues/547#issuecomment-767629149
        // for details on why this is necessary
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        let ctx: crate::Context = Default::default();
        req.extensions_mut().insert(ctx);

        inner.call(req)
    }
}