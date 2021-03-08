#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Config {
    pub(crate) config_file: Option<String>,
    pub(crate) log_level: Option<i8>,
    pub(crate) database_url: Option<String>,
    pub(crate) database_migrate: Option<bool>,
    pub(crate) server_host: Option<String>,
    pub(crate) server_port: Option<u32>,
}

impl Config {
    //set the method's context
    pub(super) fn new() -> Config {
        Config {
            config_file: None,
            log_level: None,
            database_url: None,
            database_migrate: None,
            server_host: None,
            server_port: None,
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
        let server_host = match self
            .server_host
            .as_ref()
            .or_else(|| src.server_host.as_ref())
        {
            Some(p) => Some(p.clone()),
            None => None,
        };
        let server_port = self.server_port.or_else(|| src.server_port);
        Config {
            config_file,
            log_level,
            database_url,
            database_migrate,
            server_host,
            server_port,
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
            server_host: Some(String::from("server_host")),
            server_port: Some(8080),
        }
    }
    fn gen_config_with_data_2() -> Config {
        Config {
            config_file: Some(String::from("2")),
            log_level: Some(1),
            database_url: Some(String::from("2")),
            database_migrate: Some(false),
            server_host: Some(String::from("2")),
            server_port: Some(2),
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
