use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "users")]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
}

impl Clone for User {
    fn clone(&self) -> Self {
        return Self {
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            username: self.username.clone(),
        };
    }
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "users")]
pub struct UserCreation {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub user_password: String,
}

impl Clone for UserCreation {
    fn clone(&self) -> Self {
        return Self {
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            username: self.username.clone(),
            user_password: self.user_password.clone(),
        };
    }
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "users")]
pub struct UserWithId {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub user_password: String,
}

impl Clone for UserWithId {
    fn clone(&self) -> Self {
        return Self {
            id: self.id,
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            username: self.username.clone(),
            user_password: self.user_password.clone(),
        };
    }
}
