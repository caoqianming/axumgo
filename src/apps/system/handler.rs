// use sqlx::{PgPool, query_as};
// use uuid::Uuid;
// use chrono::{DateTime, Utc, TimeZone};
// use serde::Serialize;
// use sqlx::types::chrono::{NaiveDateTime, Utc as chrono_Utc};
// use crate::apps::system::service::hash_password;


// pub async fn create_user(pool: &PgPool, input: CreateUser) -> Result<User> {
//     let default_password = "abc!0000";
//     let hashed_password = hash_password(&default_password).expect("hash密码失败");
//     let mut tx = pool.begin().await?;
//     let user_record = sqlx::query_as!(
//         User,
//         "INSERT INTO system_user (username, hashed_password, email, created_at, updated_at) 
//          VALUES ($1, $2, $3, $4, $5) RETURNING id, username, hashed_password, email, created_at, updated_at",
//         input.username,
//         hashed_password,
//         input.email,
//         chrono_Utc::from_utc_naive(&NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)), //当前时间
//         chrono_Utc::from_utc_naive(&NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0))  //当前时间
//     )
//     .fetch_one(&mut tx)
//     .await?;
    
//     tx.commit().await?;
    
//     Ok(user_record)
// }
