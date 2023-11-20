use akasha::dto::response::Response;
use axum::{response::IntoResponse, Json};
use http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("网络拥堵!")]
    NetWorkError,

    #[error("服务器错误! {0}")]
    ServerError(String),

    #[error("未登录!")]
    NotLogin,

    #[error("参数不合法!")]
    GetParamFailed,

    #[error("不存在此记录!")]
    NoLedger,

    #[error("不允许操作!")]
    Forbidden,

    #[error("family不存在!")]
    NoFamily,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::NetWorkError => (StatusCode::INTERNAL_SERVER_ERROR, Json(Response::fail(500, &self.to_string()))).into_response(),
            Error::NotLogin => (StatusCode::UNAUTHORIZED, Json(Response::fail(401, &self.to_string()))).into_response(),
            Error::GetParamFailed | Error::NoLedger => (StatusCode::BAD_REQUEST, Json(Response::fail(400, &self.to_string()))).into_response(),
            Error::ServerError(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Response::fail(500, &self.to_string()))).into_response(),
            Error::Forbidden | Error::NoFamily => (StatusCode::FORBIDDEN, Json(Response::fail(403, &self.to_string()))).into_response(),
        }
    }
}