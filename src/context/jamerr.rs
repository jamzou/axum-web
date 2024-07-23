use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::{res_code, res_wrapper::ResWrapper};
// use tracing::log::error;
pub enum AppErr {
    ///参数错误
    ParamError(String),
    ///权限错误
    AuthError(String),
    ///业务错误
    BizError(String),
    ///其他错误
    Other(anyhow::Error),
}

impl IntoResponse for AppErr {
    fn into_response(self) -> Response {
        let http_code = match &self {
            AppErr::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::OK,
        };
        let res: ResWrapper<String> = match self {
            AppErr::ParamError(msg) => ResWrapper::fail(res_code::PARAM_ERROR, msg),
            AppErr::AuthError(msg) => ResWrapper::fail(res_code::AUTH_ERROR, msg),
            AppErr::BizError(msg) => ResWrapper::fail(res_code::BIZ_ERROR, msg),
            AppErr::Other(e) => ResWrapper::fail(res_code::OTHER, e.to_string()),
        };
        (http_code, res).into_response()
    }
}

impl<E> From<E> for AppErr
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::Other(err.into())
    }
}
