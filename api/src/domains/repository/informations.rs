//! ドメイン Inforamation object and trait

use super::users::UserId;

use anyhow::Result;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct InformationId {
    pub id: i64,
}

impl InformationId {
    pub fn get(&self) -> i64 {
        self.id
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Information {
    pub id: InformationId,
    pub user_id: UserId,
    pub url: String,
    pub tag: String,
    pub title: String,
}

impl Information {
    pub fn new(
        id: InformationId,
        user_id: UserId,
        url: String,
        tag: String,
        title: String,
    ) -> Self {
        Self {
            id,
            user_id,
            url,
            tag,
            title,
        }
    }
}

pub trait InformationRepository {
    fn read_by_id(&self, information_id: &InformationId) -> Result<Information>;
    fn read_all(&self) -> Result<Vec<Information>>;
    fn create(&self, information: &Information) -> Result<()>;
    fn update(&self, information: &Information) -> Result<()>;
    fn delete(&self, information: &InformationId) -> Result<()>;
}
