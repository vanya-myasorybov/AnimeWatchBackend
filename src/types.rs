use axum::http::StatusCode;
use bb8_redis::{RedisConnectionManager, bb8::Pool};

pub type DefaultResponse = (StatusCode, String);
pub type ConnectionPool = Pool<RedisConnectionManager>;
