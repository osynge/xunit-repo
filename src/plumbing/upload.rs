use crate::plumbing::environment::add_environment;
use crate::plumbing::project::add_project;
use crate::plumbing::run_identifier::add_run_identifier;
use crate::plumbing::test_case::add_test_case;
use crate::plumbing::test_case_class::add_test_case_class;
use crate::plumbing::test_case_error::add_test_case_error;
use crate::plumbing::test_case_failure::add_test_case_failure;
use crate::plumbing::test_case_pass::add_test_case_pass;
use crate::plumbing::test_case_skipped::add_test_case_skipped;
use crate::plumbing::test_file::add_test_file;
use crate::plumbing::test_file_run::add_test_file_run;
use crate::plumbing::test_run::add_test_run;
use crate::plumbing::test_suite::add_test_suite;
use crate::DbConnection;

use super::test_case;

pub fn get_upload(
    conn: &DbConnection,
    item: &xunit_repo_interface::Upload,
) -> Result<crate::model::project::Project, diesel::result::Error> {
    println!("got:{:#?}", item);
    let project = add_project(
        conn,
        item.project.sk.as_ref(),
        item.project.identifier.as_ref(),
        item.project.human_name.as_ref(),
    )?;
    println!("project:{:#?}", project);
    let env = add_environment(
        conn,
        item.environment.sk.as_ref(),
        Some(&item.environment.key_value),
    )?;
    println!("env:{:#?}", env);
    let run = add_run_identifier(
        conn,
        project.id,
        item.run.sk.as_ref(),
        item.run.client_identifier.as_ref(),
        None,
    )?;
    println!("run:{:#?}", run);
    let tr = add_test_run(&conn, run.id, env.id)?;
    println!("tr:{:#?}", tr);

    for file_item in item.files.iter() {
        let dir = &file_item.directory;
        let name = &file_item.filename;
        let test_file = add_test_file(conn, dir, name)?;
        let test_file_run = add_test_file_run(conn, test_file.id, tr.id)?;

        for ts in file_item.content.testsuite.iter() {
            let test_suite = add_test_suite(conn, &ts.name)?;
            for tc in ts.testcase.iter() {
                let test_case_class = add_test_case_class(conn, &tc.classname)?;
                let test_case = add_test_case(conn, &tc.name, test_case_class.id, test_suite.id)?;
                match (&tc.skipped, &tc.failure, &tc.error) {
                    (Some(skipmsg), None, None) => {
                        add_test_case_skipped(
                            conn,
                            test_file_run.id,
                            test_case.id,
                            &Some(tc.time),
                            &Some(skipmsg.clone()),
                        )?;
                    }
                    (None, Some(failmsg), None) => {
                        add_test_case_failure(
                            conn,
                            test_file_run.id,
                            test_case.id,
                            &Some(tc.time),
                            &Some(failmsg.message.clone()),
                            &Some(failmsg.failure_type.clone()),
                            &Some(failmsg.description.clone()),
                            &tc.system_out,
                            &tc.system_err,
                        )?;
                    }
                    (None, None, Some(tc_error)) => {
                        add_test_case_error(
                            conn,
                            test_file_run.id,
                            test_case.id,
                            &Some(tc.time),
                            &Some(tc_error.message.clone()),
                            &Some(tc_error.error_type.clone()),
                            &Some(tc_error.description.clone()),
                            &tc.system_out,
                            &tc.system_err,
                        )?;
                    }
                    (None, None, None) => {
                        add_test_case_pass(conn, test_file_run.id, test_case.id, &Some(tc.time))?;
                    }
                    _ => {
                        println!("Cannot mix");
                    }
                }
            }
        }
    }
    Ok(project)
}
