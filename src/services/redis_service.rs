use bb8_redis::{
    RedisConnectionManager,
    bb8::{self, PooledConnection},
    redis,
};
use redis::AsyncCommands;

use crate::{
    errors::{AppError, Result},
    types::ConnectionPool,
};

#[derive(Clone)]
pub struct RedisService {
    client: ConnectionPool,
}

impl RedisService {
    pub async fn new(redis_url: String) -> Result<Self> {
        let manager = RedisConnectionManager::new(redis_url).unwrap();

        let client = bb8::Pool::builder().build(manager).await.unwrap();

        Ok(RedisService { client })
    }
    pub async fn get<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: redis::FromRedisValue,
    {
        let mut conn = self.get_connection().await?;

        conn.get(key).await.map_err(|e| AppError::Redis(e))
    }

    pub async fn set_ex<T>(&self, key: &str, value: T, seconds: u64) -> Result<()>
    where
        T: redis::ToRedisArgs + Send + Sync,
    {
        let mut conn = self.get_connection().await?;

        conn.set_ex(key, value, seconds)
            .await
            .map_err(|e| AppError::Redis(e))
    }

    pub async fn del(&self, key: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        conn.del(key).await.map_err(|e| AppError::Redis(e))
    }

    async fn get_connection(&self) -> Result<PooledConnection<RedisConnectionManager>> {
        self.client
            .get()
            .await
            .map_err(|e| AppError::BadRequest(format!("Failed to get Redis connection: {}", e)))
    }
}
