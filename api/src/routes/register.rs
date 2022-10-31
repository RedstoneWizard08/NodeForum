use actix_web::{HttpResponse, web, put, Error};
use deadpool_postgres::{Client, Pool};

use crate::{errors::AppError, database::register::add_user, models::user::UserCreation};

#[put("/api/users")]
pub async fn register(user: web::Json<UserCreation>, db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let user_info = user.into_inner();
    let client: Client = db_pool.get().await.map_err(AppError::PoolError)?;
    let new_user = add_user(&client, user_info).await?;

    return Ok(HttpResponse::Ok().json(new_user));
}
