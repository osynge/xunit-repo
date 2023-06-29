use crate::model::bind_environment_keyvalue::{
    BindEnvironmentKeyvalue, BindEnvironmentKeyvalueJson, BindEnvironmentKeyvalueNew,
};
use crate::DbConnection;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn add_bind_environment_keyvalue(
    conn: &mut DbConnection,
    item: &BindEnvironmentKeyvalueJson,
) -> Result<BindEnvironmentKeyvalue, diesel::result::Error> {
    use crate::schema::bind_environment_keyvalue::dsl::*;
    match bind_environment_keyvalue
        .filter(fk_environment.eq(&item.fk_environment))
        .filter(fk_keyvalue.eq(&item.fk_keyvalue))
        .first::<BindEnvironmentKeyvalue>(conn)
    {
        Ok(result) => Ok(result),
        Err(_) => {
            let new_keyvalue = BindEnvironmentKeyvalueNew {
                fk_environment: item.fk_environment,
                fk_keyvalue: item.fk_keyvalue,
            };

            insert_into(bind_environment_keyvalue)
                .values(&new_keyvalue)
                .execute(conn)
                .expect("Error saving new keyvalue");

            let result = bind_environment_keyvalue
                .order(id.desc())
                .first(conn)
                .unwrap();
            Ok(result)
        }
    }
}
