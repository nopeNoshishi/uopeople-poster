//! informations ルーティング
//!  
//! CRUDを行うインターフェースを提供しています。

use super::super::handler::informations::*;

use axum::{routing::*, Router};

pub fn informations_roouter() -> Router {
    Router::new()
        .route("/info/create", post(insert_new_info))
        .route("/info/all", get(get_infos))
        .route("/info/update", post(update_info))
        .route("/info/delete", post(delete_info))
}
