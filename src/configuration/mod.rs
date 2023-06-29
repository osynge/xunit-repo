pub mod clap;
pub mod configuration;
pub mod environment;
pub mod toml;
use converge::Converge;
use thiserror::Error;
#[derive(Error, Debug)]
pub(crate) enum ConfigureErr {
    #[error("File not found '{0}'.")]
    TomlErr(#[from] toml::toml::de::Error),
    #[error("io parsing error")]
    IoErr(#[from] std::io::Error),
    #[error("File not found '{0}'.")]
    FilePathIsNotFile(String),
}
pub(crate) fn configure() -> Result<configuration::Config, ConfigureErr> {
    let cfg_clap = clap::cli_clap();
    let cfg_env = environment::cli_env();
    let cfg_clap_env = cfg_clap.converge(cfg_env);
    let cfg_file = match &cfg_clap_env.config_file {
        Some(p) => toml::load_config_from_path_string(p)?,
        None => match toml::load_config_from_default_path() {
            Ok(f) => f,
            Err(f) => configuration::Config::new(),
        },
    };
    let cfg = cfg_clap_env.converge(cfg_file);
    /*clap_fern::log_setup(&cfg);
    info!("config={:#?}", cfg);*/
    Ok(cfg)
}
