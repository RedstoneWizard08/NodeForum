use actix_web::{post, web, HttpResponse};
use deadpool_postgres::{Pool, Client};
use pbkdf2::{password_hash::{PasswordHash, PasswordVerifier}, Pbkdf2};

use crate::{database::login::{LoginInfo, get_user}, errors::AppError};

#[post("/api/users")]
pub async fn login(info: web::Json<LoginInfo>, db_pool: web::Data<Pool>) -> HttpResponse {
    let user_info = info.into_inner();
    let client: Client = db_pool.get().await.map_err(AppError::PoolError).unwrap();
    let user = get_user(&client, &user_info).await.unwrap();
    let password_from_db = user.user_password.clone();
    let parsed_hash = PasswordHash::new(&password_from_db).unwrap();

    assert!(Pbkdf2.verify_password(user_info.password.clone().as_bytes(), &parsed_hash).is_ok());

    return HttpResponse::Ok().json(&user);
}
