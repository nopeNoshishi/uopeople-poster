use crate::infrastructures::repository::informations::{InformationEntity};

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Lecture {
    url: String,
    tags: String
}

#[derive(Debug, Clone, Serialize)]
pub struct JsonInfoResponse {
    pub id: i64,
    pub url: String,
    pub tag: Option<String>,
    pub title: Option<String>,
}

impl From<InformationEntity> for JsonInfoResponse {
    fn from(query_info: InformationEntity) -> Self {
        Self {
            id: query_info.id,
            url: query_info.url,
            tag: query_info.tag,
            title: query_info.title,
        }
    }
}