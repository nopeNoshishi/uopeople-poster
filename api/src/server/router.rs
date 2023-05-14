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
        .route("/api/v1/post_info", post(insert_new_info))
        .route("/api/v1/all_infos", get(get_infos))
        .route("/api/v1/update_info", post(update_info))
        .route("/api/v1/delete_info", post(delete_info))
}
