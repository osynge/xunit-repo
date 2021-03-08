pub(super) fn cli_env() -> super::configuration::Config {
    let mut out = super::configuration::Config::new();
    for (key, value) in std::env::vars() {
        if "XR_CONFIG".eq(&key) {
            out.config_file = Some(value.clone());
        }
        if "XR_ENVIROMENT_KEY".eq(&key) {
            out.database_url = Some(value.clone());
        }
        if "XR_HOST".eq(&key) {
            out.server_host = Some(value.clone());
        }
        if "XR_PORT".eq(&key) {
            out.server_port = Some(
                value
                    .parse()
                    .expect("Environment variable XR_PORT is not an integer"),
            );
        }
    }
    out
}
