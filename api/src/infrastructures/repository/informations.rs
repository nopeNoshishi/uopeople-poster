// use super::super::database::schema::*;
// use super::super::connection;
// use crate::domains::informations::{Information, InformationId, InformationRepository};
// use diesel::prelude::*;
// use diesel::QueryDsl;
// use anyhow::Result;

// //
// // Entity
// //

// #[derive(Debug, Clone, Eq, PartialEq, Hash, Insertable)]
// #[table_name = informations]
// pub struct NewInformationEntity {
//     pub url: String,
//     pub tag: String,
// }

// impl NewInformationEntity {
//     fn from(model: &Information) -> Self {
//         Self {
//             url: model.url.to_owned(),
//             tag: model.tag.to_owned(),
//         }
//     }
// }

// #[derive(Debug, Clone, Eq, PartialEq, Hash, Queryable, Identifiable, AsChangeset)]
// #[table_name = informations]
// pub struct InformationEntity  {
//     pub id: i32,
//     pub url: String,
//     pub tag: String,
// }

// impl InformationEntity {
//     fn from(model: &Information) -> Self {
//         Self {
//             id: model.id.get(),
//             url: model.url.to_owned(),
//             tag: model.tag.to_owned(),
//         }
//     }

//     fn of(&self) -> Information {
//         Information {
//             id: InformationId::new(self.id),
//             url: self.url.to_owned(),
//             tag: self.tag.to_owned(),
//         }
//     }
// }

// pub struct InformationRepositoryImpl {}

// impl InformationRepository for InformationRepositoryImpl {
//     fn find_by_id(&self, information_id: InformationId) -> Result<Information> {
//         use super::super::database::schema::informations::dsl;

//         let conn = &mut  connection::establish_connection();
//         let entity: InformationEntity = dsl::informations
//             .filter(informations::id.eq(information_id.get()))
//             .get_result(conn)?;

//         Ok(entity.of())
//     }

//     fn list(&self) -> Result<Vec<Information>> {
//         use super::super::database::schema::informations::dsl;

//         let query = dsl::informations.into_boxed();
//         let conn = &mut connection::establish_connection();
//         let results: Vec<InformationEntity> = query.limit(100).load(conn)?;

//         Ok(results.into_iter().map(|e| e.of()).collect())
//     }

//     fn insert(&self, information: &Information) -> Result<()> {
//         use super::super::database::schema::informations::dsl;

//         let entity = NewInformationEntity::from(information);
//         let conn = &mut connection::establish_connection();
//         diesel::insert_into(dsl::informations)
//             .values(entity)
//             .execute(conn)?;

//         Ok(())
//     }

//     fn update(&self, document: &Information) -> Result<()> {
//         let entity = InformationEntity::from(document);
//         let conn = &mut connection::establish_connection();
//         diesel::update(Information::url)
//             .set(&entity)
//             .execute(&conn)?;

//         Ok(())
//     }

//     fn delete(&self, Information: &Information) -> Result<()> {
//         let entity = InformationEntity::from(Information);
//         let conn = &mut connection::establish_connection();
//         diesel::delete(&entity).execute(conn)?;

//         Ok(())
//     }
// }
