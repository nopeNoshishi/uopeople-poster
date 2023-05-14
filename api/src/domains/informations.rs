
use crate::infrastructures::repository::informations::*;
use diesel::{insert_into, update, delete};
use diesel::prelude::*;
use diesel::result::QueryResult;
use diesel::RunQueryDsl;
use crate::infrastructures::*;

pub struct InformationRepositoryImpl {
    pub database: String,
}

impl InformationRepository for InformationRepositoryImpl {
    fn find_by_id(&self, information_id: InformationId) -> QueryResult<InformationEntity> {
        use crate::infrastructures::database::schema::informations::dsl;

        let conn = &mut connection::establish_connection();

        dsl::informations.find(information_id.get()).first(conn)
    }

    fn list(&self) -> QueryResult<Vec<InformationEntity>> {
        use crate::infrastructures::database::schema::informations::dsl;

        let conn = &mut connection::establish_connection();
    
        dsl::informations.load(conn)
    }

    fn insert(&self, new_information_entity: &NewInformationEntity) -> QueryResult<i64> {
        use crate::infrastructures::database::schema::informations::dsl;

        let conn = &mut connection::establish_connection();
    
        insert_into(dsl::informations)
            .values(new_information_entity)
            .returning(dsl::id)
            .get_result(conn)
    }

    fn update(&self, new_information_entity: &NewInformationEntity) -> QueryResult<InformationEntity> {
        use crate::infrastructures::database::schema::informations::dsl;

        let conn = &mut connection::establish_connection();
            
        update(dsl::informations.find(new_information_entity.get_id().unwrap()))
            .set((
                dsl::url.eq(new_information_entity.get_url()),
                 dsl::tag.eq(new_information_entity.get_tag()),
                  dsl::title.eq(new_information_entity.get_title())))
            .get_result(conn)
    }

    fn delete(&self, information_id: &InformationId) -> QueryResult<usize> {
        use crate::infrastructures::database::schema::informations::dsl;

        let conn = &mut connection::establish_connection();
    
        delete(dsl::informations.find(information_id.get()))
            .execute(conn)
    }  
}
