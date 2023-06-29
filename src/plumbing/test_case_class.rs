use crate::model::test_case_class::{TestCaseClass, TestCaseClassNew};
use crate::DbConnection;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn add_test_case_class(
    conn: &mut DbConnection,
    new_name: &String,
) -> Result<TestCaseClass, diesel::result::Error> {
    use crate::schema::test_case_class::dsl::*;
    match test_case_class
        .filter(name.eq(new_name))
        .first::<TestCaseClass>(conn)
    {
        Ok(p) => return Ok(p),
        Err(_) => {
            let new_test_case_class_new = TestCaseClassNew {
                name: new_name.as_str(),
            };
            insert_into(test_case_class)
                .values(&new_test_case_class_new)
                .execute(conn)
                .expect("Error saving new test_case_class_new");

            let result = test_case_class.order(id.desc()).first(conn)?;
            Ok(result)
        }
    }
}
