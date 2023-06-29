use converge::Converge;

#[derive(Debug, Clone, PartialEq, Converge)]
pub(crate) struct Config {
    pub(crate) config_file: Option<String>,
    pub(crate) log_in_json: Option<bool>,
    pub(crate) log_level: Option<i8>,
    pub(crate) database_url: Option<String>,
    pub(crate) database_migrate: Option<bool>,
    pub(crate) host: Option<String>,
    pub(crate) port: Option<u32>,
    pub(crate) viewer_url: Option<String>,
}

impl Config {
    //set the method's context
    pub(super) fn new() -> Config {
        Config {
            config_file: None,
            log_in_json: None,
            log_level: None,
            database_url: None,
            database_migrate: None,
            host: None,
            port: None,
            viewer_url: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn gen_config_with_data_1() -> Config {
        Config {
            config_file: Some(String::from("config_file")),
            log_in_json: Some(true),
            log_level: Some(1),
            database_url: Some(String::from("database_url")),
            database_migrate: Some(true),
            host: Some(String::from("host")),
            port: Some(8080),
            viewer_url: Some(String::from("https://192.168.0.10:9999")),
        }
    }
    fn gen_config_with_data_2() -> Config {
        Config {
            config_file: Some(String::from("2")),
            log_in_json: Some(false),
            log_level: Some(1),
            database_url: Some(String::from("2")),
            database_migrate: Some(false),
            host: Some(String::from("2")),
            port: Some(2),
            viewer_url: Some(String::from("https://192.168.0.10:9998")),
        }
    }

    #[test]
    fn gets_default_with_none() {
        let a = gen_config_with_data_1();
        let b = Config::new();
        let c = b.converge(a.clone());
        assert_eq!(c, a);
    }

    #[test]
    fn gets_none_with_none() {
        let a = Config::new();
        let b = Config::new();
        let c = b.converge(a.clone());
        assert_eq!(c, a);
    }

    #[test]
    fn gets_original_with_none() {
        let a = gen_config_with_data_1();
        let b = Config::new();
        let c = a.clone().converge(b.clone());
        assert_eq!(c, a);
    }

    #[test]
    fn gets_original_with_some() {
        let a = gen_config_with_data_1();
        let b = gen_config_with_data_2();
        let c = a.clone().converge(b.clone());
        assert_eq!(c, a);
    }
}
