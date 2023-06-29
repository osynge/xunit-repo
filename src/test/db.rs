use converge::Converge;
use diesel::r2d2::{self, ConnectionManager};
use serde::Deserialize;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::OnceLock;

pub type DbConnection = diesel::pg::PgConnection;

pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;

pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<DbConnection>>;

static DB_URL: OnceLock<String> = OnceLock::new();
static DB_CONNECTION_POOL: OnceLock<Pool> = OnceLock::new();
static DB_CONNECTION_MUTEX: Mutex<i32> = Mutex::new(1);

#[derive(Deserialize, Debug, Converge)]
pub struct DbUrl {
    pub password: String,
    pub username: String,
    pub hostname: String,
    #[serde(default = "db_url_default_port")]
    pub port: u16,
    pub database: String,
}

impl DbUrl {
    fn as_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.hostname, self.port, self.database,
        )
    }
}

fn db_url_default_port() -> u16 {
    5432
}

pub fn db_url_request() -> String {
    let prefix = "TEST_DB_";
    let mut dburl = match envy::prefixed(prefix).from_env::<DbUrl>() {
        Ok(p) => p,
        Err(err) => match (err) {
            envy::Error::MissingValue(p) => {
                panic!(
                    "Missing environment variable: {}{}",
                    prefix,
                    p.to_uppercase()
                )
            }
            envy::Error::Custom(p) => {
                panic!("{}", p);
            }
        },
    };
    dburl.as_url()
}

pub fn url_get() -> &'static str {
    DB_URL.get_or_init(|| db_url_request())
}

fn request_connection_pool() -> Pool {
    let url = url_get();
    xunit_repo_db::establish_connection_pool(url, true).unwrap()
}
//connection_pool_get

pub fn get_connection_pool() -> &'static Pool {
    DB_CONNECTION_POOL.get_or_init(|| request_connection_pool())
}

pub fn get_pooled_connection() -> PooledConnection {
    DB_CONNECTION_POOL
        .get_or_init(|| request_connection_pool())
        .get()
        .unwrap()
}

fn get_uuid_as_string() -> String {
    let uuid_human_name = uuid::Uuid::new_v4();
    let mut long_string = uuid_human_name.to_string();
    long_string.truncate(32);
    long_string
}
