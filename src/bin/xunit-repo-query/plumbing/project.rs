use crate::model::project::{Project, ProjectNew};
use crate::DbConnection;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::{dsl::insert_into, query_builder::nodes::Identifier};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use xunit_repo::schema::project::human_name;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectJson {
    pub sk: String,
    pub identiifier: String,
    pub human_name: String,
}

pub fn get_all_project(conn: &DbConnection) -> Result<Vec<ProjectJson>, diesel::result::Error> {
    use crate::schema::project::dsl::*;
    let tmp = project
        .select((sk, identiifier, human_name))
        .order(id.desc())
        .load::<(String, String, String)>(conn)?;
    let result = tmp
        .into_iter()
        .map(|(new_sk, new_identiifier, new_human_name)| ProjectJson {
            sk: new_sk,
            identiifier: new_identiifier,
            human_name: new_human_name,
        })
        .collect();
    Ok(result)
}
