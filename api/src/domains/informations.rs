use std::marker::PhantomData;
use anyhow::Result;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Id<T> {
    id: i32,
    _phantom: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }

    pub fn get(&self) -> i32 {
        self.id
    }
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        Id::new(0)
    }
}

pub type InformationId = Id<Information>;

#[derive(Debug, Clone)]
pub struct Information {
    pub id: InformationId,
    pub url: String,
    pub tag: String,
}

impl Information {
    pub fn create(url: String, tag: String) -> Self {
        Self {
            id: Default::default(),
            url,
            tag,
        }
    }
}

pub trait InformationRepository {
    fn find_by_id(&self, information_id: InformationId) -> Result<Information>;
    fn list(&self) -> Result<Vec<Information>>;
    fn insert(&self, information: &Information) -> Result<()>;
    fn update(&self, information: &Information) -> Result<()>;
    fn delete(&self, information: &Information) -> Result<()>;
}