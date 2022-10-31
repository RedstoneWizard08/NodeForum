use serde::Deserialize;
use dotenv::dotenv;
use config::{Config, Environment};
use deadpool_postgres::Config as PostgresConfig;

#[derive(Debug, Default, Deserialize)]
pub struct RawAppConfiguration {
    pub host: String,
    pub port: i32,

    pub db_name: String,
    pub db_host: String,
    pub db_port: i32,
    pub db_user: Option<String>,
    pub db_pass: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct AppConfiguration {
    pub host: String,
    pub port: i32,

    pub db: PostgresConfig,
}

pub fn get_config() -> AppConfiguration {
    dotenv().ok();

    let config_ = Config::builder()
        .add_source(Environment::default())
        .set_default("host", "localhost".to_string()).unwrap()
        .set_default("port", 4000 as i32).unwrap()
        .set_default("db_name", "postgres").unwrap()
        .set_default("db_host", "localhost").unwrap()
        .set_default("db_port", 5432 as i32).unwrap()
        .set_default("db_user", Some("postgres")).unwrap()
        .set_default("db_pass", None::<String>).unwrap()
        .build()
        .unwrap();
    
    let raw_config: RawAppConfiguration = config_.try_deserialize().unwrap();
    let mut pg_config = PostgresConfig::new();

    pg_config.dbname = Some(raw_config.db_name);
    pg_config.host = Some(raw_config.db_host);
    pg_config.port = Some(raw_config.db_port as u16);
    pg_config.user = raw_config.db_user;
    pg_config.password = raw_config.db_pass;
    
    let config = AppConfiguration {
        host: raw_config.host,
        port: raw_config.port,

        db: pg_config,
    };

    return config;
}
