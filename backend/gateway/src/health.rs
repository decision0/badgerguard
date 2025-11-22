//! Health check endpoint

use hyper::body::Incoming;
use hyper::Response;
use http_body_util::Full;
use bytes::Bytes;
use std::convert::Infallible;

/// Health check response
pub async fn health_check() -> Result<Response<Full<Bytes>>, Infallible> {
    let response = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Full::new(Bytes::from(r#"{"status":"healthy"}"#)))
        .unwrap();
    Ok(response)
}

