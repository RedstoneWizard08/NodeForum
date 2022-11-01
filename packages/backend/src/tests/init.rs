use actix_web::{App, test, web, dev::{ServiceResponse, Service}, Error};
use actix_http::Request;
use tokio_postgres::NoTls;
use crate::{
    middleware::logger::LoggingMiddlewareFactory,
    routes::{
        handle_index,
        handle_login,
        handle_register,
    }, log::Logger, database::migrate::run_migrations, config::get_config,
};

pub struct TestingVars {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

pub async fn init_mock_server() -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    let config = get_config();
    let pool = config.db.create_pool(None, NoTls).unwrap();
    let logger = Logger::new();

    run_migrations(&pool.get().await.unwrap()).await;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(LoggingMiddlewareFactory::new(logger.clone()))
            .service(handle_index)
            .service(handle_login)
            .service(handle_register)
    ).await;

    return app;
}

pub fn setup_testing_env() {
    if is_testing_env_setup() {
        return;
    }

    // let first_name = fakeit::name::first();
    // let last_name = fakeit::name::last();
    // let username = fakeit::internet::username();
    // let mut email = username.clone();
    // let password = fakeit::password::generate(true, true, false, 32);

    // email.push_str(("@".to_string() + fakeit::internet::domain_name().as_str()).as_str());

    let first_name = "Some";
    let last_name = "Guy";
    let username = "someguy";
    let email = "someguy@testing.dev";
    let password = "HahaItsMyPasswordLOL";

    std::env::set_var("_NODEFORUM_DEV_TEST_FIRST_NAME", first_name);
    std::env::set_var("_NODEFORUM_DEV_TEST_LAST_NAME", last_name);
    std::env::set_var("_NODEFORUM_DEV_TEST_USERNAME", username);
    std::env::set_var("_NODEFORUM_DEV_TEST_EMAIL", email);
    std::env::set_var("_NODEFORUM_DEV_TEST_PASSWORD", password);
}

pub fn clear_testing_env() {
    std::env::remove_var("_NODEFORUM_DEV_TEST_FIRST_NAME");
    std::env::remove_var("_NODEFORUM_DEV_TEST_LAST_NAME");
    std::env::remove_var("_NODEFORUM_DEV_TEST_USERNAME");
    std::env::remove_var("_NODEFORUM_DEV_TEST_EMAIL");
    std::env::remove_var("_NODEFORUM_DEV_TEST_PASSWORD");
}

pub fn is_testing_env_setup() -> bool {
    let var1 = std::env::var("_NODEFORUM_DEV_TEST_FIRST_NAME");
    let var2 = std::env::var("_NODEFORUM_DEV_TEST_LAST_NAME");
    let var3 = std::env::var("_NODEFORUM_DEV_TEST_USERNAME");
    let var4 = std::env::var("_NODEFORUM_DEV_TEST_EMAIL");
    let var5 = std::env::var("_NODEFORUM_DEV_TEST_PASSWORD");

    if var1.is_ok() &&
        var2.is_ok() &&
        var3.is_ok() &&
        var4.is_ok() &&
        var5.is_ok()
    {
        return true;
    } else {
        return false;
    }
}

pub fn get_testing_vars() -> TestingVars {
    if !is_testing_env_setup() {
        setup_testing_env();
    }

    let first_name = std::env::var("_NODEFORUM_DEV_TEST_FIRST_NAME").unwrap();
    let last_name = std::env::var("_NODEFORUM_DEV_TEST_LAST_NAME").unwrap();
    let username = std::env::var("_NODEFORUM_DEV_TEST_USERNAME").unwrap();
    let email = std::env::var("_NODEFORUM_DEV_TEST_EMAIL").unwrap();
    let password = std::env::var("_NODEFORUM_DEV_TEST_PASSWORD").unwrap();

    return TestingVars {
        first_name,
        last_name,
        username,
        email,
        password,
    };
}
