use redis::{AsyncCommands, Client, aio::MultiplexedConnection};

use crate::errors::{AppError, Result};

#[derive(Clone)]
pub struct RedisService {
    client: redis::Client,
}

impl RedisService {
    pub fn new(redis_url: String) -> Result<Self> {
        let client = Client::open(redis_url)
            .map_err(|_| AppError::BadRequest(String::from("Redis is field connection")))?;
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

    async fn get_connection(&self) -> Result<MultiplexedConnection> {
        self.client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::Redis(e))
    }
}
