use axum::{
    response,
    response::IntoResponse,
    Json,
    body::{boxed, Body, BoxBody},
    http::{Request, Response, StatusCode, Uri},
};
use tower::ServiceExt;
use tower_http::services::ServeDir;
use serde::Deserialize;

// use super::response::*;

use diesel::prelude::*;
use diesel::{insert_into, delete, update};
use crate::infrastructures::*;

use crate::infrastructures::repository::users::{User, NewUser};
use crate::infrastructures::database::schema::users::dsl::*;

#[derive(Deserialize)]
pub struct LectureRequest {
    url: String,
    tags: String
}

#[derive(Deserialize)]
pub struct UserReq {
    name: String,
    email: String
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

pub async fn show_users() -> impl IntoResponse  {

    let content = serde_json::json!({
        "message": "OK".to_string()
    });

    let connection = &mut connection::establish_connection();
    let results = users
        .load::<User>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for user in results {
        println!("{}", user.name);
        println!("--------------");
        println!("{}", user.email);
    }


    Json(content)
}


pub async fn store(Json(request): Json<UserReq>) -> impl IntoResponse  {

    let entity = NewUser::new(request.name, request.email);

    let connection = &mut connection::establish_connection();
    
    
    let result = insert_into(users)
        .values(entity)
        .execute(connection);

    match result {
        Ok(_) => return (
            StatusCode::OK,
            "OK"
        ),
        Err(err) => {
            eprintln!("{:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong..."
            )
        }
    }
}

pub async fn delete_user(Json(request): Json<UserReq>) -> impl IntoResponse  {

    // let entity = NewUser::new(request.name, request.email);

    let connection = &mut connection::establish_connection();

    let data = users.select(id).filter(name.eq(request.name))
        .first::<i32>(connection);

    match data {
        Ok(num) => {
            let result = delete(users.filter(id.eq(num)))
                .execute(connection);

            match result {
                Ok(_) => return (
                    StatusCode::OK,
                    "OK"
                ),
                Err(err) => {
                    eprintln!("{:?}", err);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Something went wrong..."
                    )
                }
            }
        },
        Err(err) => {
            eprintln!("{:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong..."
            )
        }
    }



}

pub async fn update_user(Json(request): Json<UserReq>) -> impl IntoResponse  {

    let connection = &mut connection::establish_connection();
        
    let result = update(users.find(1))
        .set(name.eq("newname"))
        .get_result::<User>(connection);

    match result {
        Ok(_) => return (
            StatusCode::OK,
            "OK"
        ),
        Err(err) => {
            eprintln!("{:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong..."
            )
        }
    }
}

pub async fn healthcheck() -> impl IntoResponse {

    let content = "Thank you";

    Json(content)
}

// pub async fn update() -> impl IntoResponse {

//     let content = serde_json::json!({
//         "message": "OK".to_string()
//     });

//     Json(content)
// }
