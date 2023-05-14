use super::super::database::schema::*;
use diesel::prelude::*;
use diesel::result::QueryResult;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Insertable)]
#[table_name = "informations"]
pub struct NewInformationEntity {
    id: Option<i64>,
    url: String,
    tag: String,
    title: Option<String>,
}

impl NewInformationEntity {

    pub fn new(id: Option<i64>, url: String, tag: String, title: Option<String>) -> Self {
        Self { id, url, tag, title }
    }

    pub fn get_id(&self) -> Option<i64> {
        self.id
    }

    pub fn get_url(&self) -> String {
        self.url.to_owned()
    }

    pub fn get_tag(&self) -> String {
        self.tag.to_owned()
    }

    pub fn get_title(&self) -> Option<String> {
        self.title.to_owned()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Queryable)]
pub struct InformationEntity {
    pub id: i64,
    pub url: String,
    pub tag: Option<String>,
    pub title: Option<String>,
    pub created_at: NaiveDateTime
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct InformationId {
    pub id: i64,
}

impl InformationId {
    pub fn get(&self) -> i64 {
        self.id
    }
}

pub trait InformationRepository {
    fn find_by_id(&self, information_id: InformationId) -> QueryResult<InformationEntity>;
    fn list(&self) -> QueryResult<Vec<InformationEntity>>;
    fn insert(&self, new_information_entity: &NewInformationEntity) -> QueryResult<i64>;
    fn update(&self, new_information_entity: &NewInformationEntity) -> QueryResult<InformationEntity>;
    fn delete(&self, information: &InformationId) -> QueryResult<usize>;
}
