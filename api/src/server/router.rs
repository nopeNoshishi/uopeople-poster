//! TODO 
//! 

use axum::{
    routing::*,
    Router
};
use super::handler::*;


pub fn create_roouter() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/api/v1/healthcheck", post(healthcheck))
        .route("/api/v1/store", post(store))
}
