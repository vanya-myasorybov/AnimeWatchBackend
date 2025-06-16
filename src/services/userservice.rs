use axum::http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

use crate::{
    auth::password::PasswordService,
    dto::user::{UserInfoResponse, UserUpdateRequest},
    entity::users::{ActiveModel, Column, Entity, Model},
    errors::{AppError, Result},
    types::DefaultResponse,
};

pub struct UserService;

impl UserService {
    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        request: UserUpdateRequest,
    ) -> Result<DefaultResponse> {
        let model = Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

        let mut user: ActiveModel = model.into();

        user.avatar = Set(request.avatar);

        user.nickname = Set(request.nickname);

        user.description = Set(request.description);

        user.update(db).await?;

        Ok((StatusCode::OK, "Account is update success".to_string()))
    }
    pub async fn get_user(db: &DatabaseConnection, id: String) -> Result<UserInfoResponse> {
        let model = Self::find_user(db, &id).await?;
        Ok(UserInfoResponse::from(model))
    }

    pub async fn delete(
        username: &str,
        db: &DatabaseConnection,
        password: &str,
    ) -> Result<DefaultResponse> {
        let model = Self::find_user(db, username).await?;

        if !PasswordService::verify_password(password, &model.password_hash)? {
            return Err(AppError::BadRequest("Password not verify".to_string()));
        }

        let user: ActiveModel = model.into();

        user.delete(db).await?;

        Ok((StatusCode::OK, "Delete user account is success".to_string()))
    }

    pub async fn find_user(db: &DatabaseConnection, id: &str) -> Result<Model> {
        Entity::find()
            .filter(Column::Username.eq(id).or(Column::Email.eq(id)))
            .one(db)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))
    }
}
