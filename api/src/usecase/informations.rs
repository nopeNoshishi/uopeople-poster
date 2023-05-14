use crate::infrastructures::repository::informations::*;
use diesel::{insert_into, update, delete};
use diesel::prelude::*;
use diesel::result::QueryResult;
use diesel::RunQueryDsl;
use crate::infrastructures::*;

pub fn post_document(info: &NewInformationEntity) -> QueryResult<i64> {
    use crate::infrastructures::database::schema::informations::dsl;

    let conn = &mut connection::establish_connection();

    insert_into(dsl::informations)
        .values(info)
        .returning(dsl::id)
        .get_result(conn)
}

pub fn get_documents() -> QueryResult<Vec<InformationEntity>> {
    use crate::infrastructures::database::schema::informations::dsl;

    let conn = &mut connection::establish_connection();

    dsl::informations.load(conn)
}

pub fn update_document(
    update_id: i64, new_url: String, new_tag: String, new_title: Option<String>,
) -> QueryResult<InformationEntity> {
    use crate::infrastructures::database::schema::informations::dsl;

    let conn = &mut connection::establish_connection();
        
    update(dsl::informations.find(update_id))
        .set((dsl::url.eq(new_url), dsl::tag.eq(new_tag), dsl::title.eq(new_title.unwrap())))
        .get_result(conn)
}

pub fn delete_document(delete_id: i64) -> QueryResult<usize> {
    use crate::infrastructures::database::schema::informations::dsl;

    let conn = &mut connection::establish_connection();

    delete(dsl::informations.find(delete_id))
        .execute(conn)
}

// pub fn delete_document(
//     repository: impl DocumentRepository,
//     document_id: DocumentId,
// ) -> Result<(), Error> {
//     // ...
// }
