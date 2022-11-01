use serde::{Deserialize, Serialize};
use super::{user_settings::UserSettingsWithId, user::UserWithId};

#[derive(Deserialize, Serialize)]
pub struct FullUser {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub user_password: String,

    pub settings: UserSettingsWithId,
}

impl FullUser {
    pub fn new(user: UserWithId, settings: UserSettingsWithId) -> Self {
        return Self {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            username: user.username,
            user_password: user.user_password,
            
            settings,
        };
    }
}

impl Clone for FullUser {
    fn clone(&self) -> Self {
        return Self {
            id: self.id,
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            username: self.username.clone(),
            user_password: self.user_password.clone(),

            settings: self.settings.clone(),
        };
    }
}
