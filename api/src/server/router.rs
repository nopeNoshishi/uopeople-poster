//! TODO 
//! 

use axum::{
    routing::*,
    Router,
    http::StatusCode
};

use super::handler::*;

use tower_http::services::ServeDir;


pub fn create_roouter() -> Router {
    let static_files_service = get_service(
        ServeDir::new("public").append_index_html_on_directories(true),
    )
    .handle_error(|error: std::io::Error| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", error),
        )
    });

    Router::new()
        .fallback(static_files_service)
        .route("/", get(index))
        .route("/api/v1/healthcheck", get(healthcheck))
        .route("/api/v1/post_info", post(insert_new_info))
        .route("/api/v1/all_infos", get(get_infos))
        .route("/api/v1/update_info", post(update_info))
        .route("/api/v1/delete_info", post(delete_info))
}
