pub mod log;
pub mod util;
pub mod config;
pub mod routes;
pub mod models;
pub mod errors;
pub mod database;
pub mod middleware;

#[cfg(test)]
pub mod tests;

use database::migrate::run_migrations;
use log::{info, Logger};
use routes::{handle_index, handle_login, handle_register};
use crate::config::get_config;
use actix_web::{App, HttpServer, web};
use middleware::logger::LoggingMiddlewareFactory;
use postgres::NoTls;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let config = get_config();
    let pool = config.db.create_pool(None, NoTls).unwrap();
    let logger = Logger::new();

    run_migrations(&pool.get().await.unwrap()).await;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(LoggingMiddlewareFactory::new(logger.clone()))
            .service(handle_index)
            .service(handle_login)
            .service(handle_register)
    });

    let runner = server.bind((config.host.clone(), config.port.clone() as u16))?.run();

    info(format!("Running server on: {}:{}", config.host, config.port).as_str());

    return runner.await;
}
