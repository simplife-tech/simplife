use axum::{response::IntoResponse, Json};
use http::StatusCode;

use crate::dto::response::Response;

#[derive(Debug)]
pub enum Error {
    ServiceError(i32, &'static str),
    ServerError(String),
    NetWorkError,
    NotLogin,
    NotExistUid,
    RequestTimeout,
    BadRequest,
    UserHasNoFamily,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::ServiceError(code, msg) => (
                StatusCode::OK,
                Json(Response::fail(code, msg))
            ).into_response(),
            Error::NetWorkError => (
                StatusCode::OK,
                Json(Response::fail(-500, "网络异常"))
            ).into_response(),
            Error::NotLogin => (
                StatusCode::OK,
                Json(Response::fail(-401, "未登录"))
            ).into_response(),
            Error::NotExistUid => (
                StatusCode::OK,
                Json(Response::fail(10000, "uid错误"))
            ).into_response(),
            Error::ServerError(msg) => (
                StatusCode::OK,
                Json(Response::fail(-500, &msg))
            ).into_response(),
            Error::RequestTimeout => (
                StatusCode::REQUEST_TIMEOUT,
                Json(Response::fail(-408, "请求超时"))
            ).into_response(),
            Error::BadRequest => (
                StatusCode::BAD_REQUEST,
                Json(Response::fail(-400, "请求参数错误"))
            ).into_response(),
            Error::UserHasNoFamily => (
                StatusCode::OK,
                Json(Response::fail(-500, "尚未加入家庭"))
            ).into_response(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ServiceError(_, msg) => write!(f, "{}", msg),
            Error::NetWorkError => write!(f, "网络异常"),
            Error::NotLogin => write!(f, "未登录"),
            Error::NotExistUid => write!(f, "不存在的uid"),
            Error::ServerError(msg) => write!(f, "{}", msg),
            Error::RequestTimeout => write!(f, "请求超时"),
            Error::BadRequest => write!(f, "请求参数错误"),
            Error::UserHasNoFamily => write!(f, "尚未加入家庭"),
        }
    }
}

impl std::error::Error for Error {}