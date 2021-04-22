use super::configuration;
use serde_derive::Deserialize;
use std::path::Path;

pub(super) use toml;

#[derive(Deserialize)]
struct ConfigFile {
    log_level: Option<i8>,
    json_logs: Option<bool>,
    database_url: Option<String>,
    database_migrate: Option<bool>,
    host: Option<String>,
    port: Option<u32>,
}

impl Into<configuration::Config> for ConfigFile {
    fn into(self) -> configuration::Config {
        configuration::Config {
            config_file: None,
            log_level: self.log_level,
            log_in_json: self.json_logs,
            database_url: self.database_url,
            database_migrate: self.database_migrate,
            host: self.host,
            port: self.port,
        }
    }
}

pub(super) fn load_config_from_path_string(
    input_path: &String,
) -> Result<configuration::Config, super::ConfigureErr> {
    let path = Path::new(input_path);
    if !path.is_file() {
        return Err(super::ConfigureErr::FilePathIsNotFile(String::from(
            input_path,
        )));
    }
    let toml_str = std::fs::read_to_string(&path)?;
    let cf: ConfigFile = toml::from_str(&toml_str)?;
    Ok(cf.into())
}

pub(super) fn load_config_from_default_path() -> Result<configuration::Config, ()> {
    let path = String::from("/etc/xunit-repo.toml");
    if let Ok(cfg) = load_config_from_path_string(&path) {
        return Ok(cfg);
    };
    if let Some(mut dirhome) = dirs::home_dir() {
        dirhome.push(".xunit-repo.toml");
        if let Some(cfg_path_str) = dirhome.to_str() {
            let cfg_path = String::from(cfg_path_str);
            if let Ok(cfg) = load_config_from_path_string(&cfg_path) {
                return Ok(cfg);
            };
        }
    }
    Err(())
}
