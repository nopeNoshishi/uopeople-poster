//! インフラ Information ORM and Repoistry
//!
//! データベース連携に必要なモデルを定義し、ドメインモデルとマッピングしています。
//! また、ドメインで抽象化されたリポジトリトレイトの実装も行なっています。

use super::super::connection;
use super::super::database::schema::informations::dsl;
use super::super::database::schema::*;
use crate::domains::repository::informations::*;
use crate::domains::repository::users::UserId;

use anyhow::Result;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{delete, insert_into, update};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Insertable)]
#[diesel(table_name = informations)]
pub struct NewInformationEntity {
    user_id: i64,
    url: String,
    tag: String,
    title: String,
}

impl From<Information> for NewInformationEntity {
    fn from(information: Information) -> Self {
        Self {
            user_id: information.user_id.get(),
            url: information.url,
            tag: information.tag,
            title: information.title,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Queryable)]
pub struct InformationEntity {
    pub id: i64,
    pub user_id: i64,
    pub url: String,
    pub tag: String,
    pub title: String,
    pub created_at: NaiveDateTime,
}

impl From<InformationEntity> for Information {
    fn from(information: InformationEntity) -> Self {
        Self {
            id: InformationId { id: information.id },
            user_id: UserId {
                id: information.user_id,
            },
            url: information.url,
            tag: information.tag,
            title: information.title,
        }
    }
}

pub struct InformationRepositoryImpl {
    pub database: String,
}

impl InformationRepository for InformationRepositoryImpl {
    fn read_by_id(&self, information_id: &InformationId) -> Result<Information> {
        let conn = &mut connection::establish_connection();

        let info: InformationEntity = dsl::informations.find(information_id.get()).first(conn)?;

        Ok(info.into())
    }

    fn read_all(&self) -> Result<Vec<Information>> {
        let conn = &mut connection::establish_connection();

        let infos: Vec<InformationEntity> = dsl::informations.load(conn)?;

        Ok(infos.iter().map(|i| i.clone().into()).collect())
    }

    fn create(&self, information: &Information) -> Result<()> {
        let conn = &mut connection::establish_connection();

        insert_into(dsl::informations)
            .values(NewInformationEntity::from(information.clone()))
            .execute(conn)?;

        Ok(())
    }

    fn update(&self, information: &Information) -> Result<()> {
        let conn = &mut connection::establish_connection();

        update(dsl::informations.find(information.id.get()))
            .set((
                dsl::url.eq(information.url.to_owned()),
                dsl::tag.eq(information.tag.to_owned()),
                dsl::title.eq(information.title.to_owned()),
            ))
            .execute(conn)?;

        Ok(())
    }

    fn delete(&self, information_id: &InformationId) -> Result<()> {
        let conn = &mut connection::establish_connection();

        delete(dsl::informations.find(information_id.get())).execute(conn)?;

        Ok(())
    }
}
