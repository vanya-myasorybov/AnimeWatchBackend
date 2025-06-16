use crate::dto::regex::USERNAME_REGEX;
use serde::{Deserialize, Serialize};
use validify::Validify;

#[derive(Deserialize, Validify)]
pub struct UserUpdateRequest {
    pub avatar: String,
    #[validate(length(
        min = 3,
        max = 20,
        message = "Username must be between 3 and 20 characters"
    ))]
    pub nickname: String,
    #[validate(length(
        min = 1,
        max = 600,
        message = "Username must be between 1 and 600 characters"
    ))]
    pub description: String,
}

#[derive(Serialize)]

pub struct UserInfoResponse {
    pub nickname: String,

    pub avatar: String,

    pub description: String,

    pub username: String,
}

#[derive(Deserialize, Validify)]

pub struct UserUpdateUsernameRequest {
    #[validate(length(
        min = 3,
        max = 20,
        message = "Username must be between 3 and 20 characters"
    ))]
    pub username: String,
}

#[derive(Deserialize, Validify)]
pub struct UserPathParams {
    #[validate(length(min = 3, max = 20))]
    #[validate(regex(USERNAME_REGEX))]
    pub username: String,
}

#[derive(Deserialize, Validify)]
pub struct UserDeleteRequest {
    #[validate(length(
        min = 8,
        max = 32,
        message = "Password must be at least 8 characters long"
    ))]
    pub password: String,
}

impl From<crate::entity::users::Model> for UserInfoResponse {
    fn from(user: crate::entity::users::Model) -> Self {
        Self {
            username: user.username,
            nickname: user.nickname,
            avatar: user.avatar,
            description: user.description,
        }
    }
}
