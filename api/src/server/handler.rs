use axum::{
    response::IntoResponse,
    Json,
    body::{boxed, Body, BoxBody},
    http::{Request, Response, StatusCode, Uri},
};
use tower::ServiceExt;
use tower_http::services::ServeDir;


use tracing::log::{info, error};

use crate::infrastructures::repository::informations::{NewInformationEntity};
use crate::usecase::informations::*;
use super::response::*;
use super::request::*;

pub async fn index(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let res = get_static_file(uri.clone()).await?;

    if res.status() == StatusCode::NOT_FOUND {

        match format!("{}.html", uri).parse() {
            Ok(uri_html) => get_static_file(uri_html).await,
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid URI".to_string())),
        }
    } else {
        Ok(res)
    }
}


async fn get_static_file(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    info!("{}", uri);
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    match ServeDir::new("public").oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}

pub async fn healthcheck() -> impl IntoResponse {

    let content = "Thank you";

    Json(content)
}

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
