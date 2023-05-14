extern crate diesel;

mod domains;
mod infrastructures;
mod server;
pub mod usecase;

use std::net::SocketAddr;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;


use server::router::create_roouter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();


    // build our application with a single route
    let app = create_roouter()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
                .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on localhost;8080 -> {}", addr);

    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
