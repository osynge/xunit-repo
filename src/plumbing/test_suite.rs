use crate::model::test_suite::{TestSuite, TestSuiteNew};
use crate::DbConnection;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn add_test_suite(
    conn: &DbConnection,
    new_name: &String,
) -> Result<TestSuite, diesel::result::Error> {
    use crate::schema::test_suite::dsl::*;
    match test_suite
        .filter(name.eq(new_name))
        .first::<TestSuite>(conn)
    {
        Ok(p) => return Ok(p),
        Err(_) => {
            let new_test_run_new = TestSuiteNew { name: new_name };
            insert_into(test_suite)
                .values(&new_test_run_new)
                .execute(conn)
                .expect("Error saving new test_run_new");

            let result = test_suite.order(id.desc()).first(conn)?;
            Ok(result)
        }
    }
}
