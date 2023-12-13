use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub hashed_password: String, // 注意在实际环境中不要直接返回或输出敏感信息
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String, // 这里通常是接收明文密码，然后服务端进行hash处理
    pub email: String,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub password: Option<String>,
    pub email: Option<String>,
    // 在这里可以添加其他需要更新的字段
}