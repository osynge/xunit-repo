#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
use actix_files::Files;
use actix_web_prom::PrometheusMetrics;
use tracing_actix_web::TracingLogger;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};
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
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn level_to_tracing_level(level: &Option<i8>) -> tracing::Level {
    let default = Level::INFO;
    match level {
        Some(p) => {
            if *p < -1 {
                Level::ERROR
            } else if *p == -1 {
                Level::WARN
            } else if *p == 0 {
                Level::INFO
            } else if *p == 1 {
                Level::DEBUG
            } else if *p >= 2 {
                Level::TRACE
            } else {
                default
            }
        }
        None => default,
    }
}

fn log_level_to_env_filter(level: &Option<i8>) -> EnvFilter {
    let default = EnvFilter::new("INFO");
    match level {
        Some(p) => {
            if *p < -1 {
                EnvFilter::new("ERROR")
            } else if *p == -1 {
                EnvFilter::new("WARN")
            } else if *p == 0 {
                EnvFilter::new("INFO")
            } else if *p == 1 {
                EnvFilter::new("DEBUG")
            } else if *p >= 2 {
                EnvFilter::new("TRACE")
            } else {
                default
            }
        }
        None => default,
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let app_cfg = configuration::configure().unwrap();
    // Set up logging
    let json_logging = match app_cfg.log_in_json {
        Some(log_in_json) => log_in_json,
        None => false,
    };
    let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    LogTracer::init().expect("Unable to setup log tracer!");
    match json_logging {
        false => {
            let subscriber = FmtSubscriber::builder()
                // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
                // will be written to stdout.
                .with_max_level(level_to_tracing_level(&app_cfg.log_level))
                .with_writer(non_blocking_writer)
                // completes the builder.
                .finish();
            tracing::subscriber::set_global_default(subscriber).unwrap();
        }
        true => {
            let bunyan_formatting_layer = BunyanFormattingLayer::new(app_name, non_blocking_writer);
            let subscriber = Registry::default()
                .with(log_level_to_env_filter(&app_cfg.log_level))
                .with(JsonStorageLayer)
                .with(bunyan_formatting_layer);
            tracing::subscriber::set_global_default(subscriber).unwrap();
        }
    }
    info!("{:?}", app_cfg);
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
    let prometheus = PrometheusMetrics::new("api", Some("/metrics"), None);
    HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone())
            .wrap(TracingLogger)
            // Set a larger default json message size.
            .data(web::JsonConfig::default().limit(1024 * 1024 * 50))
            .data(database_pool.clone())
            .route("/", web::get().to(routes::home))
            .route("/health", web::get().to(routes::health))
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
