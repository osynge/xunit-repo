use crate::model::bind_environment_keyvalue::BindEnvironmentKeyvalueJson;
use crate::model::environment::{Environment, EnvironmentNew};
use crate::plumbing::bind_environment_keyvalue::add_bind_environment_keyvalue;
use crate::plumbing::keyvalue::add_keyvalue;
use crate::DbConnection;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use std::collections::HashMap;
use uuid::Uuid;

fn environment_get_by_sk_hash_keyvalue(
    conn: &DbConnection,
    filter_sk: &String,
    filter_hash_keyvalue: &String,
) -> Result<Environment, diesel::result::Error> {
    use crate::schema::environment::dsl::*;
    environment
        .filter(sk.eq(filter_sk))
        .filter(hash_keyvalue.eq(filter_hash_keyvalue))
        .first::<Environment>(conn)
}

fn environment_get_by_hash_keyvalue(
    conn: &DbConnection,
    filter_hash_keyvalue: &String,
) -> Result<Environment, diesel::result::Error> {
    use crate::schema::environment::dsl::*;
    environment
        .filter(hash_keyvalue.eq(filter_hash_keyvalue))
        .first::<Environment>(conn)
}

fn environment_get_by_sk(
    conn: &DbConnection,
    filter_sk: &String,
) -> Result<Environment, diesel::result::Error> {
    use crate::schema::environment::dsl::*;
    environment
        .filter(sk.eq(filter_sk))
        .first::<Environment>(conn)
}

fn environment_insert_sk_hash_keyvalue(
    conn: &DbConnection,
    insert_sk: &String,
    insert_hash_keyvalue: &String,
) -> Result<Environment, diesel::result::Error> {
    use crate::schema::environment::dsl::*;

    let new_link = EnvironmentNew {
        sk: &insert_sk,
        hash_keyvalue: &insert_hash_keyvalue,
        best_before: None,
    };
    insert_into(environment)
        .values(&new_link)
        .execute(conn)
        .expect("Error saving new environment");

    let result = environment.order(id.desc()).first(conn).unwrap();
    Ok(result)
}

fn environment_insert_hash_keyvalue(
    conn: &DbConnection,
    insert_hash_keyvalue: &String,
) -> Result<Environment, diesel::result::Error> {
    use crate::schema::environment::dsl::*;
    let insert_sk = Uuid::new_v4().to_simple().to_string();
    let new_link = EnvironmentNew {
        sk: &insert_sk,
        hash_keyvalue: &insert_hash_keyvalue,
        best_before: None,
    };
    insert_into(environment)
        .values(&new_link)
        .execute(conn)
        .expect("Error saving new environment");

    let result = environment.order(id.desc()).first(conn).unwrap();
    Ok(result)
}

fn keyvalue_hash_gen(keyvalue: &HashMap<String, String>) -> String {
    let mut sorted: Vec<_> = keyvalue.iter().collect();
    sorted.sort();
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    sorted.hash(&mut hasher);
    hasher.finish().to_string()
}

fn insert_environment(
    conn: &DbConnection,
    sk: &String,
    keyvalue: &HashMap<String, String>,
    keyvalue_hash: &String,
) -> Result<Environment, diesel::result::Error> {
    let keyvalue_hash = keyvalue_hash_gen(keyvalue);

    let insert = environment_insert_sk_hash_keyvalue(conn, &sk, &keyvalue_hash);
    match insert {
        Ok(env) => {
            for (key, value) in keyvalue.into_iter() {
                debug!("key:{:#?}", key);
                debug!("value:{:#?}", value);
                let added_kv = match add_keyvalue(conn, key, value) {
                    Ok(p) => p.id,
                    Err(p) => {
                        return Err(p);
                    }
                };
                let new = BindEnvironmentKeyvalueJson {
                    fk_environment: env.id,
                    fk_keyvalue: added_kv,
                };
                match add_bind_environment_keyvalue(conn, &new) {
                    Ok(_) => {}
                    Err(anerr) => return Err(anerr),
                }
            }
            Ok(env)
        }
        Err(p) => Err(p),
    }
}

pub fn add_environment(
    conn: &DbConnection,
    environment_sk: Option<&String>,
    environment_key_value: Option<&HashMap<String, String>>,
) -> Result<Environment, diesel::result::Error> {
    match (environment_sk, environment_key_value) {
        (Some(sk), Some(kv)) => {
            let keyvalue_hash = keyvalue_hash_gen(kv);
            match environment_get_by_sk_hash_keyvalue(conn, &sk, &keyvalue_hash) {
                Ok(p) => Ok(p),
                Err(_) => insert_environment(conn, sk, kv, &keyvalue_hash),
            }
        }

        (None, Some(kv)) => {
            let sk = Uuid::new_v4().to_simple().to_string();

            let keyvalue_hash = keyvalue_hash_gen(kv);
            match environment_get_by_hash_keyvalue(conn, &keyvalue_hash) {
                Ok(p) => Ok(p),
                Err(_) => insert_environment(conn, &sk, kv, &keyvalue_hash),
            }
        }
        (None, None) => Err(diesel::result::Error::NotFound),
        (Some(sk), None) => environment_get_by_sk(conn, &sk),
    }
}
