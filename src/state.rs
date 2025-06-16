use sea_orm::DatabaseConnection;

use crate::{
    auth::jwt::JwtService,
    services::{email_service::EmailService, redis_service::RedisService},
};

#[derive(Clone)]
pub struct StateRouter {
    pub db: DatabaseConnection,
    pub jwt_service: JwtService,
    pub redis_service: RedisService,
    pub email_service: EmailService,
}
