//! informations ハンドラー
//!
//! 各リクエスト(CRUD)のリクエストとリスポンスをでドメインオブジェクトに
//! マッピングしてアプリケーションに渡します。

use super::super::request::informations::*;
use super::super::response::informations::*;
use crate::application::informations::*;
use crate::domains::repository::informations::*;

use anyhow::Result;
use axum::{http::StatusCode, response::IntoResponse, Json};
use tracing::log::{debug, error, info};

pub async fn insert_new_info(
    Json(request): Json<InfoRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let new_info = Information::from(request);
    debug!("{:?}", new_info);

    let result = create(&new_info);
    debug!("{:?}", result);

    match result {
        Ok(_) => {
            let response = JsonMessage {
                message: String::from("新しい記事を追加しました！"),
            };
            Ok((StatusCode::OK, Json(response)))
        }
        Err(err) => {
            error!("{:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Error in Inseting")))
        }
    }
}

pub async fn get_infos() -> Result<impl IntoResponse, impl IntoResponse> {
    let results = read_all();
    info!("{:?}", results);

    match results {
        Ok(infos) => {
            let tmp: Vec<JsonInfoResponse> = infos
                .iter()
                .map(|info| JsonInfoResponse::from(info.clone()))
                .collect();
            Ok((StatusCode::OK, Json(tmp)))
        }
        Err(err) => {
            error!("{:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Error in Reading all"))
        }
    }
}

pub async fn update_info(
    Json(request): Json<InfoUpdateRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let new_info = Information::from(request);
    debug!("{:?}", new_info);

    let result = update(&new_info);
    debug!("{:?}", result);

    match result {
        Ok(_) => {
            let response = JsonMessage {
                message: String::from("記事を更新しました！"),
            };
            Ok((StatusCode::OK, Json(response)))
        }
        Err(err) => {
            error!("{:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Error in  Updating"))
        }
    }
}

pub async fn delete_info(
    Json(request): Json<InformationIdRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let info_id = InformationId::from(request);
    debug!("{:?}", info_id);

    let result = delete(&info_id);
    debug!("{:?}", result);

    match result {
        Ok(_) => {
            let response = JsonMessage {
                message: String::from("記事を削除しました！"),
            };
            Ok((StatusCode::OK, Json(response)))
        }
        Err(err) => {
            error!("{:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Error in  Deleting"))
        }
    }
}
