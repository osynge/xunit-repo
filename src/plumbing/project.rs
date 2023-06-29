use crate::model::project::{Project, ProjectNew};
use crate::DbConnection;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use uuid::Uuid;

fn project_get_by_sk_identifier_humanname(
    conn: &mut DbConnection,
    filter_sk: &String,
    filter_identifier: &String,
    filter_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    project
        .filter(sk.eq(filter_sk))
        .filter(identifier.eq(filter_identifier))
        .filter(human_name.eq(filter_human_name))
        .first::<Project>(conn)
}

fn project_get_by_identifier_humanname(
    conn: &mut DbConnection,
    filter_identifier: &String,
    filter_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    project
        .filter(identifier.eq(filter_identifier))
        .filter(human_name.eq(filter_human_name))
        .first::<Project>(conn)
}

fn project_get_by_sk_identifier(
    conn: &mut DbConnection,
    filter_sk: &String,
    filter_identifier: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    project
        .filter(sk.eq(filter_sk))
        .filter(identifier.eq(filter_identifier))
        .first::<Project>(conn)
}

fn project_get_by_sk_humanname(
    conn: &mut DbConnection,
    filter_sk: &String,
    filter_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    project
        .filter(sk.eq(filter_sk))
        .filter(human_name.eq(filter_human_name))
        .first::<Project>(conn)
}

fn project_get_by_humanname(
    conn: &mut DbConnection,
    filter_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    project
        .filter(human_name.eq(filter_human_name))
        .first::<Project>(conn)
}

fn project_get_by_identifier(
    conn: &mut DbConnection,
    filter_identifier: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    project
        .filter(identifier.eq(filter_identifier))
        .first::<Project>(conn)
}

fn project_get_by_sk(
    conn: &mut DbConnection,
    filter_sk: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    project.filter(sk.eq(filter_sk)).first::<Project>(conn)
}

fn project_insert_sk_identifier_humanname(
    conn: &mut DbConnection,
    insert_sk: &String,
    insert_identifier: &String,
    insert_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;

    let new_link = ProjectNew {
        sk: &insert_sk,
        identifier: &insert_identifier,
        human_name: &insert_human_name,
    };
    insert_into(project)
        .values(&new_link)
        .execute(conn)
        .expect("Error saving new project");
    let result = project.order(id.desc()).first(conn).unwrap();
    Ok(result)
}

fn project_insert_identifier_humanname(
    conn: &mut DbConnection,
    insert_identifier: &String,
    insert_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let insert_sk = Uuid::new_v4().to_simple().to_string();
    let new_link = ProjectNew {
        sk: &insert_sk,
        identifier: &insert_identifier,
        human_name: &insert_human_name,
    };
    insert_into(project)
        .values(&new_link)
        .execute(conn)
        .expect("Error saving new project");
    let result = project.order(id.desc()).first(conn).unwrap();
    Ok(result)
}

fn project_insert_sk_humanname(
    conn: &mut DbConnection,
    insert_sk: &String,
    insert_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let new_link = ProjectNew {
        sk: &insert_sk,
        identifier: &insert_human_name,
        human_name: &insert_human_name,
    };
    insert_into(project)
        .values(&new_link)
        .execute(conn)
        .expect("Error saving new project");
    let result = project.order(id.desc()).first(conn).unwrap();
    Ok(result)
}

fn project_insert_sk_identifier(
    conn: &mut DbConnection,
    insert_sk: &String,
    insert_identifier: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;

    let new_link = ProjectNew {
        sk: &insert_sk,
        identifier: &insert_identifier,
        human_name: &insert_identifier,
    };
    insert_into(project)
        .values(&new_link)
        .execute(conn)
        .expect("Error saving new project");

    let result = project.order(id.desc()).first(conn).unwrap();
    Ok(result)
}

fn project_insert_humanname(
    conn: &mut DbConnection,
    insert_human_name: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let insert_sk = Uuid::new_v4().to_simple().to_string();
    let new_link = ProjectNew {
        sk: &insert_sk,
        identifier: &insert_human_name,
        human_name: &insert_human_name,
    };
    insert_into(project)
        .values(&new_link)
        .execute(conn)
        .expect("Error saving new project");
    let result = project.order(id.desc()).first(conn).unwrap();
    Ok(result)
}

fn project_insert_identifier(
    conn: &mut DbConnection,
    insert_identifier: &String,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let insert_sk = Uuid::new_v4().to_simple().to_string();
    let new_link = ProjectNew {
        sk: &insert_sk,
        identifier: &insert_identifier,
        human_name: &insert_identifier,
    };
    insert_into(project)
        .values(&new_link)
        .execute(conn)
        .expect("Error saving new project");

    let result = project.order(id.desc()).first(conn).unwrap();
    Ok(result)
}

pub fn add_project(
    conn: &mut DbConnection,
    sk: Option<&String>,
    identifier: Option<&String>,
    human_name: Option<&String>,
) -> Result<Project, diesel::result::Error> {
    match (sk, identifier, human_name) {
        (Some(sk), Some(identifier), Some(human_name)) => {
            match project_get_by_sk_identifier_humanname(conn, &sk, &identifier, &human_name) {
                Ok(p) => Ok(p),
                Err(_) => {
                    project_insert_sk_identifier_humanname(conn, &sk, &identifier, &human_name)
                }
            }
        }
        (None, Some(identifier), Some(human_name)) => {
            match project_get_by_identifier_humanname(conn, &identifier, &human_name) {
                Ok(p) => Ok(p),
                Err(_) => project_insert_identifier_humanname(conn, &identifier, &human_name),
            }
        }
        (Some(sk), None, Some(human_name)) => {
            match project_get_by_sk_humanname(conn, &sk, &human_name) {
                Ok(p) => Ok(p),
                Err(_) => project_insert_sk_humanname(conn, &sk, &human_name),
            }
        }
        (Some(sk), Some(identifier), None) => {
            match project_get_by_sk_identifier(conn, &sk, &identifier) {
                Ok(p) => Ok(p),
                Err(_) => project_insert_sk_identifier(conn, &sk, &identifier),
            }
        }
        (None, None, Some(human_name)) => match project_get_by_humanname(conn, &human_name) {
            Ok(p) => Ok(p),
            Err(_) => project_insert_humanname(conn, &human_name),
        },
        (None, Some(identifier), None) => match project_get_by_identifier(conn, identifier) {
            Ok(p) => Ok(p),
            Err(_) => project_insert_identifier(conn, identifier),
        },
        (None, None, None) => Err(diesel::result::Error::NotFound),
        (Some(sk), None, None) => project_get_by_sk(conn, &sk),
    }
}
