use crate::model::test_run::{TestRun, TestRunNew};
use crate::DbConnection;
use chrono::Utc;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use uuid::Uuid;

pub fn add_test_run(
    conn: &mut DbConnection,
    new_fk_run_identifier: i32,
    new_fk_environment: i32,
) -> Result<TestRun, diesel::result::Error> {
    use crate::schema::test_run::dsl::*;
    match test_run
        .filter(fk_run_identifier.eq(new_fk_run_identifier))
        .filter(fk_environment.eq(new_fk_environment))
        .first::<TestRun>(conn)
    {
        Ok(p) => return Ok(p),
        Err(_) => {
            let new_sk = Uuid::new_v4().to_simple().to_string();
            let new_created = Utc::now().timestamp();

            let new_test_run_new = TestRunNew {
                sk: &new_sk,
                fk_run_identifier: new_fk_run_identifier,
                fk_environment: new_fk_environment,
                created: new_created,
            };
            insert_into(test_run)
                .values(&new_test_run_new)
                .execute(conn)
                .expect("Error saving new test_run_new");

            let result = test_run.order(id.desc()).first(conn)?;
            Ok(result)
        }
    }
}
