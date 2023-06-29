use crate::model::test_case_error::TestCaseErrorNew;
use crate::model::test_case_failure::TestCaseFailureNew;
use crate::model::test_case_pass::TestCasePassNew;
use crate::model::test_case_skipped::TestCaseSkippedNew;
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

pub fn upload_short(
    conn: &mut DbConnection,
    config: &crate::SharedConfig,
    item: &xunit_repo_interface::Upload,
) -> Result<xunit_repo_interface::UploadResponse, diesel::result::Error> {
    debug!("got:{:#?}", item);
    debug!("config:{:#?}", config);
    let project = add_project(
        conn,
        item.project.sk.as_ref(),
        item.project.identifier.as_ref(),
        item.project.human_name.as_ref(),
    )?;
    debug!("project:{:#?}", project);
    let env = add_environment(
        conn,
        item.environment.sk.as_ref(),
        Some(&item.environment.key_value),
    )?;
    debug!("env:{:#?}", env);
    let run = add_run_identifier(
        conn,
        project.id,
        item.run.sk.as_ref(),
        item.run.client_identifier.as_ref(),
        None,
    )?;
    debug!("run:{:#?}", run);
    let tr = add_test_run(conn, run.id, env.id)?;
    debug!("tr:{:#?}", tr);
    let viewer_base_url = match &config.baseurl {
        Some(p) => Some(p.clone()),
        None => None,
    };
    let output = xunit_repo_interface::UploadResponse {
        project: project.sk,
        run_identifier: run.sk,
        environment: env.sk,
        test_run: tr.sk,
        viewer_url: viewer_base_url,
    };
    info!("output:{:#?}", output);
    Ok(output)
}

pub fn get_upload(
    conn: &mut DbConnection,
    config: &crate::SharedConfig,
    item: &xunit_repo_interface::Upload,
) -> Result<xunit_repo_interface::UploadResponse, diesel::result::Error> {
    debug!("got:{:#?}", item);
    debug!("config:{:#?}", config);
    let project = add_project(
        conn,
        item.project.sk.as_ref(),
        item.project.identifier.as_ref(),
        item.project.human_name.as_ref(),
    )?;
    debug!("project:{:#?}", project);
    let env = add_environment(
        conn,
        item.environment.sk.as_ref(),
        Some(&item.environment.key_value),
    )?;
    debug!("env:{:#?}", env);
    let run = add_run_identifier(
        conn,
        project.id,
        item.run.sk.as_ref(),
        item.run.client_identifier.as_ref(),
        None,
    )?;
    debug!("run:{:#?}", run);
    let tr = add_test_run(conn, run.id, env.id)?;
    debug!("tr:{:#?}", tr);
    let viewer_base_url = match &config.baseurl {
        Some(p) => Some(p.clone()),
        None => None,
    };
    let output = xunit_repo_interface::UploadResponse {
        project: project.sk,
        run_identifier: run.sk,
        environment: env.sk,
        test_run: tr.sk,
        viewer_url: viewer_base_url,
    };
    let mut list_test_case_pass = Vec::new();
    let mut list_test_case_fail = Vec::new();
    let mut list_test_case_error = Vec::new();
    let mut list_test_case_skip = Vec::new();
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
                    (Some(skipmsg), None, None) => list_test_case_skip.push(TestCaseSkippedNew {
                        fk_test_case: test_case.id,
                        fk_test_file_run: test_file_run.id,
                        time: Some(&tc.time),
                        skipped_message: Some(&skipmsg),
                    }),
                    (None, Some(failmsg), None) => list_test_case_fail.push(TestCaseFailureNew {
                        fk_test_case: test_case.id,
                        fk_test_file_run: test_file_run.id,
                        time: Some(&tc.time),
                        failure_message: Some(&failmsg.message),
                        failure_type: Some(&failmsg.failure_type),
                        failure_description: Some(&failmsg.description),
                        system_out: tc.system_out.as_deref(),
                        system_err: tc.system_err.as_deref(),
                    }),
                    (None, None, Some(tc_error)) => list_test_case_error.push(TestCaseErrorNew {
                        fk_test_case: test_case.id,
                        fk_test_file_run: test_file_run.id,
                        time: Some(&tc.time),
                        error_message: Some(&tc_error.message),
                        error_type: Some(&tc_error.error_type),
                        error_description: Some(&tc_error.description),
                        system_out: tc.system_out.as_deref(),
                        system_err: tc.system_err.as_deref(),
                    }),
                    (None, None, None) => list_test_case_pass.push(TestCasePassNew {
                        fk_test_case: test_case.id,
                        time: Some(tc.time),
                        fk_test_file_run: test_file_run.id,
                    }),
                    _ => {
                        error!("Cannot mix");
                    }
                }
            }
        }
    }
    match crate::plumbing::test_case_skipped::add_test_case_skip_list(conn, &list_test_case_skip) {
        Ok(p) => info!("added test skipped count={:#?}", p),
        Err(p) => error!("added_skip={:#?}", p),
    };
    match crate::plumbing::test_case_error::add_test_case_error_list(conn, &list_test_case_error) {
        Ok(p) => info!("added test error count={:#?}", p),
        Err(p) => error!("added_errors={:#?}", p),
    };
    match crate::plumbing::test_case_failure::add_test_case_failure_list(conn, &list_test_case_fail)
    {
        Ok(p) => info!("added test fail count={:#?}", p),
        Err(p) => error!("added_fails={:#?}", p),
    };
    debug!("test_case_pass={:#?}", list_test_case_pass);
    match crate::plumbing::test_case_pass::add_test_case_pass_list(conn, &list_test_case_pass) {
        Ok(p) => info!("added test passes count={:#?}", p),
        Err(p) => error!("added_pass={:#?}", p),
    };
    Ok(output)
}
