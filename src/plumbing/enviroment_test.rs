use crate::db;
use crate::model::keyvalue::KeyValueNew;
use crate::DbConnection;
use diesel::dsl::insert_into;
use diesel::RunQueryDsl;
#[test]
fn create_user_with_phone_and_email() {
    use crate::schema::keyvalue::dsl::*;
    /*"PooledConnection<ConnectionManager<SqliteConnection>>"*/
    let conn = db::establish_connection_pool(&"memory://", true)
        .get()
        .unwrap();
    let fred = &conn as &DbConnection;
    let new_key = "test@email.com";
    let new_value = "1234567890";

    let new_keyvalue = KeyValueNew {
        key: new_key,
        value: new_value,
    };
    insert_into(keyvalue)
        .values(&new_keyvalue)
        .execute(fred)
        .expect("Error saving new keyvalue");
}
