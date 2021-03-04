use crate::db;
use crate::plumbing::keyvalue;

#[test]
fn add_key_value() {
    let conn = db::establish_connection_pool(&"memory://", true)
        .get()
        .unwrap();
    let key_1 = String::from("Elephant");
    let value_1 = String::from("Babar");
    let add_1 = keyvalue::add_keyvalue(&conn, &key_1, &value_1);
    assert!(add_1.is_ok());
}

#[test]
fn add_key_value_twice() {
    let conn = db::establish_connection_pool(&"memory://", true)
        .get()
        .unwrap();
    let key_1 = String::from("Elephant");
    let value_1 = String::from("Babar");
    let value_2 = String::from("Celeste");
    let add_1 = keyvalue::add_keyvalue(&conn, &key_1, &value_1);
    assert!(add_1.is_ok());
    let add_2 = keyvalue::add_keyvalue(&conn, &key_1, &value_1);
    assert!(add_2.is_ok());
    let id_1 = add_1.unwrap().id;
    let id_2 = add_2.unwrap().id;
    assert!(id_1 == id_2);
    let add_3 = keyvalue::add_keyvalue(&conn, &key_1, &value_2);
    assert!(add_3.is_ok());
    let id_3 = add_3.unwrap().id;
    assert!(id_1 != id_3);
}
