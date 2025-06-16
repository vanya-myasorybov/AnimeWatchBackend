use crate::dto::regex::USERNAME_REGEX;
use serde::{Deserialize, Serialize};
use validify::Validify;

#[derive(Debug, Deserialize, Validify)]
pub struct SendEmailCodeRequest {
    #[modify(lowercase)]
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, Validify)]
pub struct RegisterRequest {
    #[modify(lowercase)]
    #[validate(regex(USERNAME_REGEX))]
    #[validate(length(
        min = 3,
        max = 20,
        message = "Username must be between 3 and 50 characters"
    ))]
    pub username: String,

    #[modify(lowercase)]
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(
        min = 8,
        max = 32,
        message = "Password must be at least 8 characters long"
    ))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Validify)]
pub struct LoginRequest {
    #[modify(lowercase)]
    #[validate(length(min = 1, max = 100, message = "Username/email is required"))]
    pub username_or_email: String,

    #[validate(length(min = 1, max = 32, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize)]

pub struct AuthResponse {
    pub token: String,

    pub user: UserInfo,
}

#[derive(Debug, Serialize)]

pub struct UserInfo {
    pub id: i32,

    pub username: String,

    pub email: String,
}

impl From<crate::entity::users::Model> for UserInfo {
    fn from(user: crate::entity::users::Model) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
        }
    }
}
