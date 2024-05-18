use bb8_redis::{
    bb8::{Pool, PooledConnection},
    RedisConnectionManager,
};
use std::error::Error;
use tokio::sync::OnceCell;

static CONNECTION_POOL: OnceCell<Pool<RedisConnectionManager>> = OnceCell::const_new();

async fn init_pool() -> Result<(), Box<dyn Error>> {
    let redis_host = std::env::var("REDIS_HOST").unwrap_or("localhost".to_string());
    let redis_port = std::env::var("REDIS_PORT").unwrap_or("6379".to_string());
    let redis_db = std::env::var("REDIS_DB").unwrap_or("0".to_string());

    let pool = Pool::builder()
        .build(RedisConnectionManager::new(format!("redis://{redis_host}:{redis_port}/{redis_db}"))?)
        .await?;
    CONNECTION_POOL.get_or_init(|| async { pool }).await;
    Ok(())
}

pub async fn get_connection(
) -> Result<PooledConnection<'static, RedisConnectionManager>, Box<dyn Error>> {
    if !CONNECTION_POOL.initialized() {
        init_pool().await?;
    }
    Ok(CONNECTION_POOL.get().unwrap().get().await?)
}

#[cfg(test)]
mod test {
    use super::get_connection;
    use bb8_redis::redis::AsyncCommands;

    #[tokio::test]
    async fn test_1() {
        let mut conn = get_connection().await.unwrap();

        let key = "key";
        let value = "value".to_string();

        // Stores a key-value pair to Redis.
        conn.set::<&str, String, String>(key, value.clone())
            .await
            .unwrap();

        // Gets the stored value.
        let stored_value = conn.get::<&str, String>(key).await.unwrap();

        // Check if `value` and `stored_value` match
        assert_eq!(value, stored_value);
    }

    #[tokio::test]
    async fn test_2() {
        let mut conn = get_connection().await.unwrap();
        let key = "a";
        let value = "a".to_string();
        conn.set::<&str, String, String>(key, value.clone())
            .await
            .unwrap();
        let stored_value = conn.get::<&str, String>(key).await.unwrap();
        assert_eq!(value, stored_value);
    }

    #[tokio::test]
    async fn test_3() {
        let mut conn = get_connection().await.unwrap();
        let key = "b";
        let value = "b".to_string();
        conn.set::<&str, String, String>(key, value.clone())
            .await
            .unwrap();
        let stored_value = conn.get::<&str, String>(key).await.unwrap();
        assert_eq!(value, stored_value);
    }

    #[tokio::test]
    async fn test_4() {
        let mut conn = get_connection().await.unwrap();
        let key = "c";
        let value = "c".to_string();
        conn.set::<&str, String, String>(key, value.clone())
            .await
            .unwrap();
        let stored_value = conn.get::<&str, String>(key).await.unwrap();
        assert_eq!(value, stored_value);
    }

    #[tokio::test]
    async fn test_5() {
        let mut conn = get_connection().await.unwrap();
        let key = "d";
        let value = "d".to_string();
        conn.set::<&str, String, String>(key, value.clone())
            .await
            .unwrap();
        let stored_value = conn.get::<&str, String>(key).await.unwrap();
        assert_eq!(value, stored_value);
    }

    #[tokio::test]
    async fn test_6() {
        let mut conn = get_connection().await.unwrap();
        let key = "e";
        let value = "e".to_string();
        conn.set::<&str, String, String>(key, value.clone())
            .await
            .unwrap();
        let stored_value = conn.get::<&str, String>(key).await.unwrap();
        assert_eq!(value, stored_value);
    }

    #[tokio::test]
    async fn test_7() {
        let mut conn = get_connection().await.unwrap();
        let key = "f";
        let value = "f".to_string();
        conn.set::<&str, String, String>(key, value.clone())
            .await
            .unwrap();
        let stored_value = conn.get::<&str, String>(key).await.unwrap();
        assert_eq!(value, stored_value);
    }

    #[tokio::test]
    async fn test_8() {
        let mut conn = get_connection().await.unwrap();
        let key = "g";
        let value = "g".to_string();
        conn.set::<&str, String, String>(key, value.clone())
            .await
            .unwrap();
        let stored_value = conn.get::<&str, String>(key).await.unwrap();
        assert_eq!(value, stored_value);
    }

    #[tokio::test]
    async fn test_9() {
        let mut conn = get_connection().await.unwrap();
        let key = "h";
        let value = "h".to_string();
        conn.set::<&str, String, String>(key, value.clone())
            .await
            .unwrap();
        let stored_value = conn.get::<&str, String>(key).await.unwrap();
        assert_eq!(value, stored_value);
    }

    #[tokio::test]
    async fn test_10() {
        let mut conn = get_connection().await.unwrap();
        let key = "i";
        let value = "i".to_string();
        conn.set::<&str, String, String>(key, value.clone())
            .await
            .unwrap();
        let stored_value = conn.get::<&str, String>(key).await.unwrap();
        assert_eq!(value, stored_value);
    }
}
