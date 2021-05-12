use crate::model::environment::EnvironmentJson;
use crate::model::keyvalue::KeyValueJson;
use crate::model::project::ProjectJson;
use crate::model::run_identifier::RunIdentifierJson;
use crate::model::test_case_error::TestCaseErrorJson;
use crate::model::test_case_failure::TestCaseFailureJson;
use crate::model::test_case_pass::TestCasePassJson;
use crate::model::test_case_skipped::TestCaseSkippedJson;
use crate::plumbing::environment::add_environment;
use crate::plumbing::keyvalue::add_keyvalue;
use crate::plumbing::project::add_project;
use crate::plumbing::run_identifier::add_run_identifier;
use crate::plumbing::test_case_error::add_test_case_error;
use crate::plumbing::test_case_failure::add_test_case_failure;
use crate::plumbing::test_case_pass::add_test_case_pass;
use crate::plumbing::test_case_skipped::add_test_case_skipped;
use crate::plumbing::upload::get_upload;
use crate::Pool;
use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpResponse};
use anyhow::Result;

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn home() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/index.html")))
}

pub async fn project_add(
    pool: web::Data<Pool>,
    item: web::Json<ProjectJson>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let project = item.into_inner();
    Ok(web::block(move || {
        add_project(
            &conn,
            project.sk.as_ref(),
            project.identifier.as_ref(),
            project.human_name.as_ref(),
        )
    })
    .await
    .map(|project| HttpResponse::Created().json(project))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn keyvalue_add(
    pool: web::Data<Pool>,
    item: web::Json<KeyValueJson>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let keyvalue = item.into_inner();
    Ok(
        web::block(move || add_keyvalue(&conn, &keyvalue.key, &keyvalue.value))
            .await
            .map(|project| HttpResponse::Created().json(project))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

pub async fn environment_add(
    pool: web::Data<Pool>,
    item: web::Json<EnvironmentJson>,
) -> Result<HttpResponse, Error> {
    let environment = item.into_inner();
    let conn = pool.get().unwrap();
    Ok(web::block(move || {
        add_environment(
            &conn,
            environment.sk.as_ref(),
            environment.key_value.as_ref(),
        )
    })
    .await
    .map(|project| HttpResponse::Created().json(project))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn run_add(
    pool: web::Data<Pool>,
    item: web::Json<RunIdentifierJson>,
) -> Result<HttpResponse, Error> {
    let run_identifier = item.into_inner();
    let conn = pool.get().unwrap();
    Ok(web::block(move || {
        add_run_identifier(
            &conn,
            1,
            run_identifier.sk.as_ref(),
            run_identifier.client_identifier.as_ref(),
            run_identifier.created,
        )
    })
    .await
    .map(|project| HttpResponse::Created().json(project))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn test_case_error_add(
    pool: web::Data<Pool>,
    item: web::Json<TestCaseErrorJson>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let test_case_error = item.into_inner();
    Ok(web::block(move || {
        add_test_case_error(
            &conn,
            1,
            1,
            &test_case_error.time,
            &test_case_error.error_message,
            &test_case_error.error_type,
            &test_case_error.error_description,
            &test_case_error.system_out,
            &test_case_error.system_err,
        )
    })
    .await
    .map(|project| HttpResponse::Created().json(project))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn test_case_failure_add(
    pool: web::Data<Pool>,
    item: web::Json<TestCaseFailureJson>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let run_identifier = item.into_inner();
    Ok(web::block(move || {
        add_test_case_failure(
            &conn,
            1,
            1,
            &run_identifier.time,
            &run_identifier.failure_message,
            &run_identifier.failure_type,
            &run_identifier.failure_description,
            &run_identifier.system_out,
            &run_identifier.system_err,
        )
    })
    .await
    .map(|project| HttpResponse::Created().json(project))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn test_case_skipped_add(
    pool: web::Data<Pool>,
    item: web::Json<TestCaseSkippedJson>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let run_identifier = item.into_inner();
    Ok(web::block(move || {
        add_test_case_skipped(
            &conn,
            1,
            1,
            &run_identifier.time,
            &run_identifier.skipped_message,
        )
    })
    .await
    .map(|project| HttpResponse::Created().json(project))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn test_case_pass_add(
    pool: web::Data<Pool>,
    item: web::Json<TestCasePassJson>,
) -> Result<HttpResponse, Error> {
    let run_identifier = item.into_inner();
    let conn = pool.get().unwrap();
    Ok(
        web::block(move || add_test_case_pass(&conn, 1, 1, &run_identifier.time))
            .await
            .map(|project| HttpResponse::Created().json(project))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

pub async fn upload(
    pool: web::Data<Pool>,
    shared_config: web::Data<crate::SharedConfig>,
    item: web::Json<xunit_repo_interface::Upload>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let config = shared_config.into_inner();
    let run_identifier = item.into_inner();
    Ok(
        web::block(move || get_upload(&conn, config.as_ref(), &run_identifier))
            .await
            .map(|project| HttpResponse::Created().json(project))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http;
    use actix_web::test;
    use actix_web::{http::header, web, App};
    use diesel::r2d2::ConnectionManager;
    use diesel::SqliteConnection;

    #[actix_rt::test]
    async fn test_index() {
        let database_url = "foo.db";
        let database_pool = Pool::builder()
            .build(ConnectionManager::<SqliteConnection>::new(database_url))
            .unwrap();
        let mut app = test::init_service(
            App::new()
                .data(database_pool.clone())
                .route("/homessss", web::post().to(home)),
        )
        .await;
        let ti = r#"{ "sk": "mykey", "identifier": "identifier2", "human_name" : "sdfsdfsf" }"#;

        //let req = test::TestRequest::post().uri("/").to_request();
        let req = test::TestRequest::post()
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(ti)
            .uri("/homessss")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        //assert!(resp.status().is_client_error());

        assert_eq!(resp.status(), http::StatusCode::OK);
    }
    /*
    #[actix_rt::test]
    async fn test_index_post() {
        let database_url = "foo.db";
        let database_pool = Pool::builder()
            .build(ConnectionManager::<SqliteConnection>::new(database_url))
            .unwrap();
        let mut app = test::init_service(
            App::new()
                .data(database_pool.clone())
                .route("/addlink", web::post().to(add_link_noop)),
        )
        .await;
        let ti = r#"{ "sk": "mykey", "identifier": "identifier2", "human_name" : "sdfsdfsf" }"#
            .as_bytes();
        //let req = test::TestRequest::post().uri("/").to_request();
        let req = test::TestRequest::post()
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(ti)
            .uri("/addlink")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        //assert!(resp.status().is_client_error());

        assert_eq!(resp.status(), 201);
    }
    */
}
