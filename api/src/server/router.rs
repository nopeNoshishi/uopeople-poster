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
        .route("/api/v1/healthcheck", get(healthcheck))
        .route("/api/v1/store", post(store))
        .route("/api/v1/delete", post(delete_user))
        .route("/api/v1/update", post(update_user))
        .route("/api/v1/show", post(show_users))
}

