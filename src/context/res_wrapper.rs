use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResWrapper<T> {
    pub code: u8,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> IntoResponse for ResWrapper<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let json_string = serde_json::to_string(&self);
        match json_string {
            Ok(res_str) => (StatusCode::OK, res_str).into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "json转换异常").into_response(),
        }
    }
}

impl<T> ResWrapper<T> {
    pub fn success(data: T) -> Self {
        ResWrapper {
            code: super::res_code::SUCCESS,
            msg: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn fail(code: u8, msg: String) -> Self {
        ResWrapper {
            code,
            msg,
            data: None,
        }
    }
}
