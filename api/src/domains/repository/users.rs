//! ドメイン Users object and trait

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct UserId {
    pub id: i64,
}

impl UserId {
    pub fn get(&self) -> i64 {
        self.id
    }
}
