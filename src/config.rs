#[derive(Deserialize, Debug)]
pub struct Config {

    #[serde(default = "default_log_level")]
    pub log_level: String,

    pub id: String,

    #[serde(default = "default_amqp_host")]
    pub amqp_host: String,

    pub queue: String,
    #[serde(default = "default_prefetch_count")]
    pub prefetch_count: u16,

    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_password: String,

}

fn default_log_level() -> String { "INFO".into() }
fn default_amqp_host() -> String { "amqp://127.0.0.1:5672/%2f".into() }
fn default_prefetch_count() -> u16 { 10 }
