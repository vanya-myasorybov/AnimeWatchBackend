use axum::http::StatusCode;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use uuid::Uuid;

use crate::{
    auth::{jwt::JwtService, password::PasswordService},
    dto::auth::{AuthResponse, LoginRequest, RegisterRequest, UserInfo},
    entity::users::{ActiveModel, Column, Entity},
    errors::{AppError, Result},
    services::{
        email_service::EmailService, redis_service::RedisService, userservice::UserService,
    },
    types::DefaultResponse,
};

pub struct AuthService;

impl AuthService {
    pub async fn register(
        db: &DatabaseConnection,
        jwt: &JwtService,
        redis: &RedisService,
        reguest: RegisterRequest,
        email: &EmailService,
    ) -> Result<AuthResponse> {
        if Entity::find()
            .filter(
                Column::Username
                    .eq(&reguest.username)
                    .or(Column::Email.eq(&reguest.email)),
            )
            .one(db)
            .await?
            .is_some()
        {
            return Err(AppError::BadRequest("User already exists".to_string()));
        }

        let password_hash = PasswordService::hash_password(&reguest.password)?;

        let model = ActiveModel {
            id: NotSet,
            username: Set(reguest.username.clone()),
            email: Set(reguest.email),
            nickname: Set(reguest.username),
            password_hash: Set(password_hash),
            avatar: Set("".to_string()),
            is_verified: NotSet,
            email_verified_at: NotSet,
            last_login_at: NotSet,
            created_at: NotSet,
            updated_at: NotSet,
            description: NotSet,
        };

        let user = model.insert(db).await?;

        let token = jwt.create_token(user.id, &user.username, &user.email)?;

        Self::send_email_verify(&user.email, redis, email).await?;

        Ok(AuthResponse {
            token,
            user: UserInfo::from(user),
        })
    }

    pub async fn verify_email(
        token: &str,
        redis: &RedisService,
        db: &DatabaseConnection,
    ) -> Result<()> {
        let email: String = redis
            .get(token)
            .await?
            .ok_or_else(|| AppError::NotFound("Token not found".to_string()))?;

        let user = UserService::find_user(db, &email).await?;

        if user.is_verified {
            return Err(AppError::BadRequest("User already verified".to_string()));
        }

        if user.email != email {
            return Err(AppError::BadRequest("Invalid token".to_string()));
        }

        let mut active_model: ActiveModel = user.into();

        active_model.is_verified = Set(true);

        active_model.update(db).await?;

        redis.del(token).await?;

        Ok(())
    }

    pub async fn send_email_verify(
        email: &str,
        redis: &RedisService,
        email_service: &EmailService,
    ) -> Result<DefaultResponse> {
        let token = Self::generate_uuid();

        redis.set_ex(&token, email, 14000).await?;

        email_service.send_html(email, &token).await?;

        Ok((StatusCode::OK, "Send code is success".to_string()))
    }

    pub async fn login(
        db: &DatabaseConnection,
        jwt_service: &JwtService,
        request: LoginRequest,
    ) -> Result<AuthResponse> {
        let user = Entity::find()
            .filter(
                Column::Username
                    .eq(&request.username_or_email)
                    .or(Column::Email.eq(&request.username_or_email)),
            )
            .one(db)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

        if !PasswordService::verify_password(&request.password, &user.password_hash)? {
            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        if !user.is_verified {
            return Err(AppError::Unauthorized(
                "Account email is not verify".to_string(),
            ));
        }

        let token = jwt_service.create_token(user.id, &user.username, &user.email)?;

        Ok(AuthResponse {
            token,
            user: UserInfo::from(user),
        })
    }

    #[inline]
    fn generate_uuid() -> String {
        return Uuid::new_v4().to_string();
    }
}
