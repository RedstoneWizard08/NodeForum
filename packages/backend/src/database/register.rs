use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{models::{user::{UserWithId, UserCreation}, user_settings::UserSettingsWithId, full_user::FullUser}, errors::AppError, util::password::hash};

pub async fn add_user(client: &Client, user_info: UserCreation) -> Result<FullUser, AppError> {
    let _stmt = include_str!("../sql/users/add.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();

    let password = hash(&user_info.user_password);

    let res = client.query(&stmt, &[
        &user_info.first_name,
        &user_info.last_name,
        &user_info.email,
        &password,
        &user_info.username,
    ])
        .await?.iter()
        .map(|row| UserWithId::from_row_ref(row).unwrap())
        .collect::<Vec<UserWithId>>().pop();
    
    let user = res.clone().unwrap();
    
    let _stmt2 = include_str!("../sql/users/link_settings.sql");
    let stmt2 = client.prepare(&_stmt2).await.unwrap();

    let res2 = client.query(&stmt2, &[
        &user.id,
    ])
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
