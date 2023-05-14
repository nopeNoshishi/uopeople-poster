use serde::Deserialize;

#[derive(Deserialize)]
pub struct InfoRequest {
    pub url: String,
    pub tag: String,
    pub title: Option<String>,
}

#[derive(Deserialize)]
pub struct InfoUpdateRequest {
    pub id: i64,
    pub url: Option<String>,
    pub tag: Option<String>,
    pub title: Option<String>,
}

#[derive(Deserialize)]
pub struct InfoDeleteRequest {
    pub id: i64,
}