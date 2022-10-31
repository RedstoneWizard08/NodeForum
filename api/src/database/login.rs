use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use serde::{Deserialize, Serialize};

use crate::{models::{full_user::FullUser, user::UserWithId, user_settings::UserSettingsWithId}, errors::AppError};

#[derive(Deserialize, Serialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

pub async fn get_user(client: &Client, user_info: &LoginInfo) -> Result<FullUser, AppError> {
    let _stmt = include_str!("../sql/users/get.sql");
    let _stmt = _stmt.replace("$username", user_info.username.as_str());
    let stmt = client.prepare(&_stmt).await.unwrap();

    let res = client.query(&stmt, &[])
        .await?.iter()
        .map(|row| UserWithId::from_row_ref(row).unwrap())
        .collect::<Vec<UserWithId>>().pop();

    let user = res.clone().unwrap();

    let _stmt2 = include_str!("../sql/users/get_settings.sql");
    let _stmt2 = _stmt2.replace("$user_id", user.id.to_string().as_str());
    let stmt2 = client.prepare(&_stmt2).await.unwrap();

    let res2 = client.query(&stmt2, &[])
        .await?.iter()
        .map(|row| UserSettingsWithId::from_row_ref(row).unwrap())
        .collect::<Vec<UserSettingsWithId>>().pop();
    
    if res2.is_some() {
        let full = FullUser::new(user, res2.unwrap());

        return Ok(full);
    } else {
        return Err(AppError::NotFound);
    }
}
