use crate::domains::repository::informations::*;
use crate::domains::repository::users::UserId;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct InfoRequest {
    pub user_id: i64,
    pub url: String,
    pub tag: String,
    pub title: String,
}

impl From<InfoRequest> for Information {
    fn from(request: InfoRequest) -> Information {
        Information {
            id: InformationId { id: 0 },
            user_id: UserId {
                id: request.user_id,
            },
            url: request.url,
            tag: request.tag,
            title: request.title,
        }
    }
}

#[derive(Deserialize)]
pub struct InfoUpdateRequest {
    pub id: i64,
    pub url: String,
    pub tag: String,
    pub title: String,
}

impl From<InfoUpdateRequest> for Information {
    fn from(request: InfoUpdateRequest) -> Information {
        Information {
            id: InformationId { id: request.id },
            user_id: UserId { id: 0 },
            url: request.url,
            tag: request.tag,
            title: request.title,
        }
    }
}

#[derive(Deserialize)]
pub struct InformationIdRequest {
    pub id: i64,
}

impl From<InformationIdRequest> for InformationId {
    fn from(request: InformationIdRequest) -> InformationId {
        InformationId { id: request.id }
    }
}
