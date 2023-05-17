//! Server ビルド
//!
//! 各ミドルウェアと各ルータをビルドし起動する。

use super::middleware::*;
use super::router::*;

use axum::{response::IntoResponse, routing::*, Json, Router};
use std::net::SocketAddr;

#[tokio::main]
pub async fn run() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let app = Router::new()
        .route("/healthcheck", get(healthcheck))
        .merge(basic::basic())
        .merge(staticfile::static_roouter())
        .nest("/api/v1", informations::informations_roouter())
        .layer(trace_log::trace_middleware());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on localhost;8080 -> {}", addr);

    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn healthcheck() -> impl IntoResponse {
    let content = "Thank you";

    Json(content)
}
