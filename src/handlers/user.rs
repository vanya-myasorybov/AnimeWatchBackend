use axum::extract::Path;
use axum::{Extension, Json, extract::State};
use axum_valid::Validated;

use crate::dto::user::{UserDeleteRequest, UserInfoResponse, UserPathParams};
use crate::services::authservice::AuthService;
use crate::types::DefaultResponse;
use crate::{
    StateRouter, auth::middleware::AuthenticatedUser, dto::user::UserUpdateRequest, errors::Result,
    services::userservice::UserService,
};

pub async fn update_account(
    State(StateRouter { db, .. }): State<StateRouter>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Validated(Json(request)): Validated<Json<UserUpdateRequest>>,
) -> Result<DefaultResponse> {
    UserService::update(&db, auth_user.user_id, request).await
}

pub async fn delete(
    State(StateRouter { db, .. }): State<StateRouter>,
    Extension(AuthenticatedUser { username, .. }): Extension<AuthenticatedUser>,
    Validated(Json(UserDeleteRequest { password })): Validated<Json<UserDeleteRequest>>,
) -> Result<DefaultResponse> {
    UserService::delete(&username, &db, &password).await
}

pub async fn send_email_verify(
    State(StateRouter {
        redis_service,
        email_service,
        ..
    }): State<StateRouter>,
    Extension(AuthenticatedUser { email, .. }): Extension<AuthenticatedUser>,
) -> Result<DefaultResponse> {
    AuthService::send_email_verify(&email, &redis_service, &email_service).await
}

pub async fn get_user(
    State(StateRouter { db, .. }): State<StateRouter>,
    Validated(Path(UserPathParams { username })): Validated<Path<UserPathParams>>,
) -> Result<Json<UserInfoResponse>> {
    let response = UserService::get_user(&db, username).await?;
    Ok(Json(response))
}
