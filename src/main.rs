#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

use xunit_repo_db::db;
use xunit_repo_db::model;
use xunit_repo_db::schema;
mod configuration;
mod plumbing;
mod routes;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

pub type DbConnection = SqliteConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let app_cfg = configuration::configure().unwrap();
    println!("{:?}", app_cfg);
    let database_url = match app_cfg.database_url {
        Some(url) => url,
        None => {
            let custom_error =
                std::io::Error::new(std::io::ErrorKind::Other, "No database_url specified");
            return Err(custom_error);
        }
    };
    let host = match app_cfg.host {
        Some(host) => host,
        None => {
            let custom_error = std::io::Error::new(std::io::ErrorKind::Other, "No host specified");
            return Err(custom_error);
        }
    };
    let port = match app_cfg.port {
        Some(port) => port,
        None => {
            let custom_error = std::io::Error::new(std::io::ErrorKind::Other, "No port specified");
            return Err(custom_error);
        }
    };

    let bind = format!("{}:{}", host, port);

    let migrate = match app_cfg.database_migrate {
        Some(database_migrate) => database_migrate,
        None => false,
    };

    let database_pool = match db::establish_connection_pool(&database_url, migrate) {
        Ok(pool) => pool,
        Err(err) => {
            let custom_error = std::io::Error::new(std::io::ErrorKind::Other, err);
            return Err(custom_error);
        }
    };
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
    .bind(bind)?
    .run()
    .await
}
