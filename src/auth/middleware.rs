use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::TypedHeader;
use axum_extra::headers::{Authorization, authorization::Bearer};

use crate::state::StateRouter;

#[derive(Clone)]
pub struct AuthenticatedUser {
    pub user_id: i32,
    pub username: String,
    pub email: String,
}

pub async fn auth_middleware(
    State(StateRouter { jwt_service, .. }): State<StateRouter>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = authorization.token();

    match jwt_service.validate_token(token) {
        Ok(claims) => {
            let user_id = claims
                .claims
                .sub
                .parse::<i32>()
                .map_err(|_| StatusCode::UNAUTHORIZED)?;

            let auth_user = AuthenticatedUser {
                user_id,
                username: claims.claims.username,
                email: claims.claims.email,
            };
            request.extensions_mut().insert(auth_user);
            Ok(next.run(request).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
