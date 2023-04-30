use axum::{
    extract,
    response,
    response::IntoResponse,
    Json,
    body::{boxed, Body, BoxBody},
    http::{Request, Response, StatusCode, Uri},
};
use tower::ServiceExt;
use tower_http::services::ServeDir;
use serde::Deserialize;

use super::response::*;

#[derive(Deserialize)]
pub struct LectureRequest {
    url: String,
    tags: String
}

pub async fn index(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let res = get_static_file(uri.clone()).await?;

    if res.status() == StatusCode::NOT_FOUND {
        // try with `.html`
        // TODO: handle if the Uri has query parameters
        match format!("{}.html", uri).parse() {
            Ok(uri_html) => get_static_file(uri_html).await,
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid URI".to_string())),
        }
    } else {
        Ok(res)
    }
}


async fn get_static_file(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // When run normally, the root is the workspace root
    match ServeDir::new("public").oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}

pub async fn store(extract::Json(post): extract::Json<LectureRequest>) -> response::Json<Lecture> {
    response::Json(Lecture::new(post.url, post.tags))
}

pub async fn healthcheck() -> impl IntoResponse {

    let content = serde_json::json!({
        "message": "OK".to_string()
    });

    Json(content)
}

// pub async fn update() -> impl IntoResponse {

//     let content = serde_json::json!({
//         "message": "OK".to_string()
//     });

//     Json(content)
// }
