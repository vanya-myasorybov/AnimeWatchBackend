use auth::jwt::JwtService;
use axum::middleware;
use axum::routing::{delete, post, put};
use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::auth::middleware::auth_middleware;
use crate::handlers::user;
use crate::services::email_service::EmailService;
use crate::services::redis_service::RedisService;
use crate::state::StateRouter;

pub mod auth;
pub mod config;
pub mod database;
pub mod dto;
pub mod entity;
pub mod errors;
pub mod handlers;
pub mod services;
pub mod state;

pub mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let config = config::Config::from_env()?;

    let db = database::establish_connection(config.database_url).await?;

    let jwt_service = JwtService::new(&config.jwt_secret, config.jwt_expiration);

    let redis_service = RedisService::new(config.redis_url)?;

    let email_service = EmailService::new(config.email_config)?;

    let app = create_app(db, jwt_service, redis_service, email_service);

    let listener = tokio::net::TcpListener::bind(&config.server_host).await?;

    info!("Server running on {}", &config.server_host);

    axum::serve(listener, app).await?;

    Ok(())
}

#[inline]
fn create_app(
    db: DatabaseConnection,
    jwt_service: JwtService,
    redis_service: RedisService,
    email_service: EmailService,
) -> Router {
    let state = StateRouter {
        db,
        jwt_service,
        redis_service,
        email_service,
    };

    let auth_router = Router::new()
        .route("/register", post(handlers::auth::register))
        .route("/login", post(handlers::auth::login))
        .route("/verify/:token", post(handlers::auth::verify));

    let user_router = Router::new()
        .route("/", put(user::update_account))
        .route("/:id", post(user::get_user))
        .route("/", delete(user::delete));

    let private_router =
        Router::new()
            .nest("/user", user_router)
            .layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            ));

    Router::new()
        .route("/", get(async || "HelloWorld"))
        .nest("/auth", auth_router)
        .merge(private_router)
        .with_state(state)
        .layer(CorsLayer::permissive())
}
