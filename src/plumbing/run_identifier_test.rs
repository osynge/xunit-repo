use crate::plumbing::project;
use crate::plumbing::run_identifier;
use crate::test::db::get_pooled_connection;

#[test]
fn add_run_identifier() {
    let mut conn = get_pooled_connection();
    let project_sk = Some(String::from(""));
    let project_identifier = Some(String::from(""));
    let project_humanname = Some(String::from(""));
    let add_project = project::add_project(
        &mut conn,
        project_sk.as_ref(),
        project_identifier.as_ref(),
        project_humanname.as_ref(),
    )
    .unwrap();
    let run_sk = Some(String::from(""));
    let run_client_identifier = Some(String::from(""));
    let run_created = None;

    let add_1 = run_identifier::add_run_identifier(
        &mut conn,
        add_project.id,
        run_sk.as_ref(),
        run_client_identifier.as_ref(),
        run_created,
    )
    .unwrap();
    let add_2 = run_identifier::add_run_identifier(
        &mut conn,
        add_project.id,
        run_sk.as_ref(),
        run_client_identifier.as_ref(),
        run_created,
    )
    .unwrap();
    println!("add_1={:#?}", add_1);
    println!("add_2={:#?}", add_2);
    assert!(add_1.id == add_2.id);
}
