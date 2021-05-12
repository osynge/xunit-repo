use clap::App;
use clap::Arg;
use clap::{crate_name, crate_version};

use std::convert::TryFrom;

fn log_setup(verbose_occurrences: u64, quiet_occurrences: u64) -> Option<i8> {
    if (0, 0) == (verbose_occurrences, quiet_occurrences) {
        return None;
    };
    let verbose = match i8::try_from(verbose_occurrences) {
        Ok(p) => p,
        Err(_) => i8::MAX,
    };
    let quiet = match i8::try_from(quiet_occurrences) {
        Ok(p) => p,
        Err(_) => i8::MAX,
    };
    Some(verbose.saturating_sub(quiet))
}

pub(super) fn cli_clap() -> super::configuration::Config {
    let application = App::new(crate_name!())
        .about("Parses an input file to do awesome things")
        .version(crate_version!())
        .author("Owen Synge <osynge@googlemail.com>")
        .arg(
            Arg::with_name("verbose")
                .help("Increase log output.")
                .short("v")
                .multiple(true)
                .long("verbose"),
        )
        .arg(
            Arg::with_name("quiet")
                .help("Decrease log output.")
                .short("q")
                .multiple(true)
                .long("quiet"),
        )
        .arg(
            Arg::with_name("database_url")
                .long("database-url")
                .value_name("DATABASE_URL")
                .help("Database connection URL")
                .multiple(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("database_migrate")
                .long("database-migrate")
                .help("Create or migrate Database")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("no_database_migrate")
                .long("no-database-migrate")
                .help("Do not create or migrate Database")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("logging_json")
                .long("json-logging")
                .help("Log messages in json")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("logging_line")
                .long("line-logging")
                .help("Log messages as lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("host")
                .long("host")
                .value_name("SERVER")
                .help("Sets the host to upload the xunit to.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .value_name("PORT")
                .help("Sets the host's port.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("viewer_url")
                .long("viewer-url")
                .value_name("VIEWER")
                .help("Sets the base url to view web pages.")
                .takes_value(true),
        );

    let matches = application.get_matches();
    let log_level = log_setup(
        matches.occurrences_of("verbose"),
        matches.occurrences_of("quiet"),
    );
    let config_file = match matches.value_of("config") {
        Some(p) => Some(String::from(p)),
        None => None,
    };
    let database_url = match matches.value_of("database_url") {
        Some(p) => Some(String::from(p)),
        None => None,
    };
    let database_migrate = match (
        matches.is_present("database_migrate"),
        matches.is_present("no_database_migrate"),
    ) {
        (true, true) => None,
        (true, false) => Some(true),
        (false, true) => Some(false),
        (false, false) => None,
    };
    let log_in_json = match (
        matches.is_present("logging_json"),
        matches.is_present("logging_line"),
    ) {
        (true, true) => None,
        (true, false) => Some(true),
        (false, true) => Some(false),
        (false, false) => None,
    };
    let host = match matches.value_of("host") {
        Some(p) => Some(String::from(p)),
        None => None,
    };
    let port = match matches.value_of("port") {
        Some(p) => match p.parse() {
            Ok(f) => Some(f),
            Err(_) => None,
        },
        None => None,
    };
    let viewer_url = match matches.value_of("viewer_url") {
        Some(p) => Some(String::from(p)),
        None => None,
    };
    crate::configuration::configuration::Config {
        config_file,
        log_in_json,
        log_level,
        database_url,
        database_migrate,
        host,
        port,
        viewer_url,
    }
}
