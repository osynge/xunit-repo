use crate::model::test_case_pass::{TestCasePass, TestCasePassNew};
use crate::DbConnection;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn add_test_case_pass(
    conn: &DbConnection,
    filter_fk_test_file_run: i32,
    new_fk_test_case: i32,
    tc_time: &Option<f32>,
) -> Result<TestCasePass, diesel::result::Error> {
    use crate::schema::test_case_pass::dsl::*;
    match test_case_pass
        .filter(fk_test_case.eq(new_fk_test_case))
        .filter(time.eq(tc_time))
        .filter(fk_test_file_run.eq(filter_fk_test_file_run))
        .first::<TestCasePass>(conn)
    {
        Ok(result) => Ok(result),
        Err(_) => {
            let new_keyvalue = TestCasePassNew {
                fk_test_case: new_fk_test_case,
                time: tc_time.clone(),
                fk_test_file_run: filter_fk_test_file_run,
            };

            insert_into(test_case_pass)
                .values(&new_keyvalue)
                .execute(conn)
                .expect("Error saving new test case pass");

            let result = test_case_pass.order(id.desc()).first(conn).unwrap();
            Ok(result)
        }
    }
}

pub fn add_test_case_pass_list(
    conn: &DbConnection,
    pass: &Vec<TestCasePassNew>,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::test_case_pass::dsl::*;
    diesel::insert_into(test_case_pass)
        .values(pass)
        .on_conflict((fk_test_case, fk_test_file_run))
        .do_nothing()
        .execute(conn)
}
