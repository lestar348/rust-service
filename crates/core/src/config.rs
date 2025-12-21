/// Basic application configuration stub.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub transport: String,
    pub logging_level: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            transport: "mock".to_string(),
            logging_level: "info".to_string(),
        }
    }
}
