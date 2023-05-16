//! informations ハンドラー
//! 
//! 各リクエスト(CRUD)のインターフェースです。

use axum::{
    response::IntoResponse,
    Json,
    http::StatusCode,
};

use tracing::log::{info, error};

use crate::infrastructures::repository::informations::{NewInformationEntity};
use crate::application::informations::*;
use super::super::response::informations::*;
use super::super::request::informations::*;

pub async fn insert_new_info(Json(request): Json<InfoRequest>) ->impl IntoResponse {

    let new_info = NewInformationEntity::from(request);

    let result = post_document(&new_info);

    match result {
        Ok(info_id) => return (
            StatusCode::OK,
            Json(info_id)
        ),
        Err(err) => {
            eprintln!("{:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(0)
            )
        }
    }
}

pub async fn get_infos() -> Result<impl IntoResponse, impl IntoResponse>  {

    let results = get_documents();

    match results {
        Ok(infos) => { 

            let tmp: Vec<JsonInfoResponse> = infos.iter().map(|info| JsonInfoResponse::from(info.clone()) ).collect();
            
            info!("{:?}", infos);

            Ok ((StatusCode::OK,  Json(tmp) ))
        }
        Err(err) => {
            error!("{:?}", err);
            Err (( StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong..."))
        }
    }
}

pub async fn update_info(Json(request): Json<InfoUpdateRequest>) -> Result<impl IntoResponse, impl IntoResponse>  {

    // let new_entity = NewInformationEntity::from(request);
    let result = update_document(
        request.id, request.url, request.tag, request.title
    );

    match result {
        Ok(info) => { 

            let json = JsonInfoResponse::from(info);
            info!("Inserted : {:?}", json);
            
            Ok ((StatusCode::OK,  Json(json)))
        }
        Err(err) => {
            error!("{:?}", err);
            Err (( StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong..."))
        }
    }
}

pub async fn delete_info(Json(request): Json<InformationIdRequest>) -> Result<impl IntoResponse, impl IntoResponse>  {

    let result = delete_document(request.id);

    match result {
        Ok(num) => { 
            match num {
                1 => info!("Delete!"),
                _ => info!("No content!"),
            }
            
            
            Ok ((StatusCode::OK,  Json(num)))
        }
        Err(err) => {
            error!("{:?}", err);
            Err (( StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong..."))
        }
    }
}
