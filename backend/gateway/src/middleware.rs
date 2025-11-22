//! Authentication and authorization middleware

use tower::Service;
use std::task::{Context, Poll};
use hyper::Request;
use hyper::Response;
use http_body_util::Full;
use bytes::Bytes;

/// Authentication middleware
pub struct AuthMiddleware<S> {
    inner: S,
}

impl<S> AuthMiddleware<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, B> Service<Request<B>> for AuthMiddleware<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        // TODO: Extract session cookie
        // TODO: Validate session with auth service
        // TODO: Check policy with policy engine
        // TODO: Forward request if authorized
        self.inner.call(req)
    }
}

