use std::env;
use redis::{Client, Connection};

pub async fn redis_connect() -> redis::RedisResult<Connection> {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = Client::open(redis_url)?;
    let con = client.get_connection()?;
    Ok(con)
}