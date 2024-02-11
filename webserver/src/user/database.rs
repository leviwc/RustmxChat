use sea_orm::*;

use crate::entities::user;

pub struct UserDatabase;

impl UserDatabase {
    pub async fn create_user(db: &DbConn, username: String, password: String) -> Result<(), DbErr> {
        user::ActiveModel {
            username: Set(username),
            password_hash: Set(password),
            ..Default::default()
        }
        .save(db)
        .await?;

        Ok(())
    }

    pub async fn get_user_for_login(
        db: &DbConn,
        username: String,
        password: String,
    ) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find()
            .filter(user::Column::Username.eq(username))
            .filter(user::Column::PasswordHash.eq(password))
            .one(db)
            .await
    }
}
