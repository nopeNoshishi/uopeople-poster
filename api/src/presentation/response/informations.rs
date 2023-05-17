use crate::domains::repository::informations::*;

use serde::Serialize;

#[derive(Serialize)]
pub struct JsonMessage {
    pub message: String,
}

#[derive(Serialize)]
pub struct JsonInfoResponse {
    pub id: i64,
    pub url: String,
    pub tag: String,
    pub title: String,
}

impl From<Information> for JsonInfoResponse {
    fn from(query_info: Information) -> Self {
        Self {
            id: query_info.id.get(),
            url: query_info.url,
            tag: query_info.tag,
            title: query_info.title,
        }
    }
}
