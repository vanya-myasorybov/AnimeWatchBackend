use crate::{
    StateRouter,
    dto::auth::{AuthResponse, LoginRequest, RegisterRequest},
    errors::Result,
    services::authservice::AuthService,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use axum_valid::Validated;

pub async fn register(
    State(StateRouter {
        db,
        jwt_service,
        redis_service,
        email_service,
    }): State<StateRouter>,
    Validated(Json(request)): Validated<Json<RegisterRequest>>,
) -> Result<(StatusCode, Json<AuthResponse>)> {
    let response =
        AuthService::register(&db, &jwt_service, &redis_service, request, &email_service).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn verify(
    State(StateRouter {
        db, redis_service, ..
    }): State<StateRouter>,
    Path(token): Path<String>,
) -> Result<StatusCode> {
    AuthService::verify_email(&token, &redis_service, &db).await?;
    Ok(StatusCode::OK)
}

pub async fn login(
    State(StateRouter {
        db, jwt_service, ..
    }): State<StateRouter>,
    Validated(Json(request)): Validated<Json<LoginRequest>>,
) -> Result<Json<AuthResponse>> {
    let response = AuthService::login(&db, &jwt_service, request).await?;
    Ok(Json(response))
}
