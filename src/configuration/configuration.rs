#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Config {
    pub(crate) config_file: Option<String>,
    pub(crate) log_level: Option<i8>,
    pub(crate) database_url: Option<String>,
    pub(crate) database_migrate: Option<bool>,
    pub(crate) host: Option<String>,
    pub(crate) port: Option<u32>,
}

impl Config {
    //set the method's context
    pub(super) fn new() -> Config {
        Config {
            config_file: None,
            log_level: None,
            database_url: None,
            database_migrate: None,
            host: None,
            port: None,
        }
    }
    pub(super) fn copy_with_default(&self, src: &Config) -> Config {
        let config_file = match self
            .config_file
            .as_ref()
            .or_else(|| src.config_file.as_ref())
        {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let log_level = self.log_level.or_else(|| src.log_level);
        let database_url = match self
            .database_url
            .as_ref()
            .or_else(|| src.database_url.as_ref())
        {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let database_migrate = self.database_migrate.or_else(|| src.database_migrate);
        let host = match self
            .host
            .as_ref()
            .or_else(|| src.host.as_ref())
        {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let port = self.port.or_else(|| src.port);
        Config {
            config_file,
            log_level,
            database_url,
            database_migrate,
            host,
            port,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn gen_config_with_data_1() -> Config {
        Config {
            config_file: Some(String::from("config_file")),
            log_level: Some(1),
            database_url: Some(String::from("database_url")),
            database_migrate: Some(true),
            host: Some(String::from("host")),
            port: Some(8080),
        }
    }
    fn gen_config_with_data_2() -> Config {
        Config {
            config_file: Some(String::from("2")),
            log_level: Some(1),
            database_url: Some(String::from("2")),
            database_migrate: Some(false),
            host: Some(String::from("2")),
            port: Some(2),
        }
    }

    #[test]
    fn gets_default_with_none() {
        let a = gen_config_with_data_1();
        let b = Config::new();
        let c = b.copy_with_default(&a);
        assert_eq!(c, a);
    }

    #[test]
    fn gets_none_with_none() {
        let a = Config::new();
        let b = Config::new();
        let c = b.copy_with_default(&a);
        assert_eq!(c, a);
    }

    #[test]
    fn gets_original_with_none() {
        let a = gen_config_with_data_1();
        let b = Config::new();
        let c = a.copy_with_default(&b);
        assert_eq!(c, a);
    }

    #[test]
    fn gets_original_with_some() {
        let a = gen_config_with_data_1();
        let b = gen_config_with_data_2();
        let c = a.copy_with_default(&b);
        assert_eq!(c, a);
    }
}
