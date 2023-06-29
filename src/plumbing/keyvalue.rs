use crate::model::keyvalue::{KeyValue, KeyValueNew};
use crate::DbConnection;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn add_keyvalue(
    conn: &mut DbConnection,
    new_key: &String,
    new_value: &String,
) -> Result<KeyValue, diesel::result::Error> {
    use crate::schema::keyvalue::dsl::*;
    match keyvalue
        .filter(key.eq(new_key))
        .filter(value.eq(new_value))
        .first::<KeyValue>(conn)
    {
        Ok(result) => Ok(result),
        Err(_) => {
            let new_keyvalue = KeyValueNew {
                key: &new_key,
                value: &new_value,
            };

            insert_into(keyvalue)
                .values(&new_keyvalue)
                .execute(conn)
                .expect("Error saving new keyvalue");

            let result = keyvalue.order(id.desc()).first(conn).unwrap();
            Ok(result)
        }
    }
}
