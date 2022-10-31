use actix_http::Method;
use actix_web::{
    http::{self, header::ContentType},
    test,
};
use crate::{tests::init::{init_mock_server, get_testing_vars}, database::login::LoginInfo, models::user::UserCreation};

#[actix_web::test]
pub async fn test_login_ok() {
    let vars = get_testing_vars();

    let first_name = vars.first_name;
    let last_name = vars.last_name;
    let username = vars.username;
    let email = vars.email;
    let password = vars.password;

    let data = UserCreation {
        first_name,
        last_name,
        username,
        email,
        user_password: password,
    };

    let app = init_mock_server().await;
    let req = test::TestRequest::default()
        .uri("/api/users")
        .method(Method::PUT)
        .insert_header(ContentType::json())
        .set_json(data)
        .to_request();
    
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);

    let vars = get_testing_vars();

    let username = vars.username;
    let password = vars.password;

    let data = LoginInfo {
        username,
        password,
    };

    let req = test::TestRequest::default()
        .uri("/api/users")
        .method(Method::POST)
        .insert_header(ContentType::json())
        .set_json(data)
        .to_request();
    
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);
}
