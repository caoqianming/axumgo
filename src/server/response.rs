use serde::{Serialize, Deserialize};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Serialize)]
pub struct ErrInfo {
    pub err_code: String,
    pub err_msg: String,
    pub err_data: Option<serde_json::Value>,
}