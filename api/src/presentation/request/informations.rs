use crate::infrastructures::repository::informations::{NewInformationEntity};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct InfoRequest {
    pub url: String,
    pub tag: String,
    pub title: Option<String>,
}

impl From<InfoRequest> for NewInformationEntity {
    fn from(request: InfoRequest) -> NewInformationEntity {
        NewInformationEntity::new(
            None,
            request.url,
            request.tag,
            request.title
        )
    }
}


#[derive(Deserialize)]
pub struct InfoUpdateRequest {
    pub id: i64,
    pub url: String,
    pub tag: String,
    pub title: Option<String>,
}

impl From<InfoUpdateRequest> for NewInformationEntity {
    fn from(request: InfoUpdateRequest) -> NewInformationEntity {
        NewInformationEntity::new(
            Some(request.id),
            request.url,
            request.tag,
            request.title
        )
    }
}

#[derive(Deserialize)]
pub struct InformationIdRequest {
    pub id: i64,
}