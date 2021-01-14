use crate::model::test_case_skipped::{TestCaseSkipped, TestCaseSkippedJson, TestCaseSkippedNew};
use crate::Pool;
use actix_web::web;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn add_test_case_skipped(
    pool: web::Data<Pool>,
    filter_fk_test_file_run: i32,
    new_fk_test_case: i32,
    new_time: &Option<f32>,
    new_skipped_message: &Option<String>,
) -> Result<TestCaseSkipped, diesel::result::Error> {
    use crate::schema::test_case_skipped::dsl::*;
    let db_connection = pool.get().unwrap();
    match test_case_skipped
        .filter(fk_test_case.eq(new_fk_test_case))
        .filter(time.eq(new_time))
        .filter(skipped_message.eq(new_skipped_message))
        .filter(fk_test_file_run.eq(filter_fk_test_file_run))
        .first::<TestCaseSkipped>(&db_connection)
    {
        Ok(result) => Ok(result),
        Err(_) => {
            let new_keyvalue = TestCaseSkippedNew {
                fk_test_case: new_fk_test_case,
                time: new_time.clone(),
                skipped_message: new_skipped_message.as_deref(),
                fk_test_file_run: filter_fk_test_file_run,
            };

            insert_into(test_case_skipped)
                .values(&new_keyvalue)
                .execute(&db_connection)
                .expect("Error saving new test case skipped");

            let result = test_case_skipped
                .order(id.desc())
                .first(&db_connection)
                .unwrap();
            Ok(result)
        }
    }
}
