use crate::model::run_identifier::{RunIdentifier, RunIdentifierNew};
use crate::DbConnection;
use chrono::Utc;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use uuid::Uuid;

fn run_identifier_get_by_sk_client_identifier(
    conn: &DbConnection,
    filter_fk_project: i32,
    filter_sk: &String,
    filter_client_identifier: &String,
) -> Result<RunIdentifier, diesel::result::Error> {
    use crate::schema::run_identifier::dsl::*;
    run_identifier
        .filter(sk.eq(filter_sk))
        .filter(fk_project.eq(filter_fk_project))
        .filter(client_identifier.eq(filter_client_identifier))
        .first::<RunIdentifier>(conn)
}

fn run_identifier_get_by_client_identifier(
    conn: &DbConnection,
    filter_fk_project: i32,
    filter_client_identifier: &String,
) -> Result<RunIdentifier, diesel::result::Error> {
    use crate::schema::run_identifier::dsl::*;
    run_identifier
        .filter(client_identifier.eq(filter_client_identifier))
        .filter(fk_project.eq(filter_fk_project))
        .first::<RunIdentifier>(conn)
}

fn run_identifier_get_by_sk(
    conn: &DbConnection,
    filter_fk_project: i32,
    filter_sk: &String,
) -> Result<RunIdentifier, diesel::result::Error> {
    use crate::schema::run_identifier::dsl::*;
    run_identifier
        .filter(sk.eq(filter_sk))
        .filter(fk_project.eq(filter_fk_project))
        .first::<RunIdentifier>(conn)
}

fn insert_run_identifier(
    conn: &DbConnection,
    insert_fk_project: i32,
    insert_created: i64,
    insert_sk: &String,
    insert_client_identifier: &String,
) -> Result<RunIdentifier, diesel::result::Error> {
    use crate::schema::run_identifier::dsl::*;
    let new_runident = RunIdentifierNew {
        sk: &insert_sk,
        client_identifier: insert_client_identifier,
        created: insert_created,
        fk_project: insert_fk_project,
    };
    insert_into(run_identifier)
        .values(&new_runident)
        .execute(conn)
        .expect("Error saving new run_identifier");
    let result = run_identifier.order(id.desc()).first(conn)?;
    Ok(result)
}

pub fn add_run_identifier(
    conn: &DbConnection,
    fk_project: i32,
    run_sk: Option<&String>,
    run_client_identifier: Option<&String>,
    run_created: Option<i64>,
) -> Result<RunIdentifier, diesel::result::Error> {
    let created = match run_created {
        Some(p) => p,
        None => Utc::now().timestamp(),
    };
    debug!("run:created:{:#?}", created);
    debug!("run:run_sk:{:#?}", run_sk);
    debug!("run:run_client_identifier:{:#?}", run_client_identifier);
    match (run_sk, run_client_identifier) {
        (Some(sk), Some(client_identifier)) => {
            debug!("run:1");
            match run_identifier_get_by_sk_client_identifier(
                conn,
                fk_project,
                &sk,
                &client_identifier,
            ) {
                Ok(p) => Ok(p),
                Err(_) => insert_run_identifier(conn, fk_project, created, sk, client_identifier),
            }
        }
        (None, Some(client_identifier)) => {
            debug!("run:client_identifier:{:#?}", client_identifier);
            let sk = Uuid::new_v4().to_simple().to_string();

            match run_identifier_get_by_client_identifier(conn, fk_project, &client_identifier) {
                Ok(p) => Ok(p),
                Err(_) => insert_run_identifier(conn, fk_project, created, &sk, client_identifier),
            }
        }
        (None, None) => {
            debug!("run:2");
            Err(diesel::result::Error::NotFound)
        }
        (Some(sk), None) => {
            debug!("run:3");
            run_identifier_get_by_sk(conn, fk_project, &sk)
        }
    }
}
