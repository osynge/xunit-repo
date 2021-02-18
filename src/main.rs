#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod db;
mod model;
mod plumbing;
mod routes;
mod schema;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

pub type DbConnection = SqliteConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let database_pool = db::establish_connection();
    HttpServer::new(move || {
        App::new()
            .data(database_pool.clone())
            .route("/", web::get().to(routes::home))
            .route("/project_add", web::post().to(routes::project_add))
            .route("/keyvalue_add", web::post().to(routes::keyvalue_add))
            .route("/environment_add", web::post().to(routes::environment_add))
            .route("/run_add", web::post().to(routes::run_add))
            .route(
                "/test_case_error_add",
                web::post().to(routes::test_case_error_add),
            )
            .route(
                "/test_case_failure_add",
                web::post().to(routes::test_case_failure_add),
            )
            .route(
                "/test_case_skipped_add",
                web::post().to(routes::test_case_skipped_add),
            )
            .route(
                "/test_case_pass_add",
                web::post().to(routes::test_case_pass_add),
            )
            .route("/upload", web::post().to(routes::upload))
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}
