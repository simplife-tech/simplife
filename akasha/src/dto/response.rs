use axum::{response::IntoResponse, Json};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub data: Option<T>,
    pub message: Option<String>,
    pub code: i32,
}

impl <T>IntoResponse for Response<T>
where
    T: Serialize, 
{
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl Response<String> {
    pub fn success() -> Response<String> {
        Response {
            data: None,
            message: Some("success".to_string()),
            code: 0,
        }
    }
    pub fn fail(code: i32, message: &str) -> Response<String> {
        Response {
            data: None,
            message: Some(message.to_string()),
            code,
        }
    }
    pub fn network_error() -> Response<String> {
        Response {
            data: None,
            message: Some("网络拥堵".to_string()),
            code: -500,
        }
    }
    pub fn login_error() -> Response<String> {
        Response {
            data: None,
            message: Some("用户名或密码错误".to_string()),
            code: -401,
        }
    }
    pub fn not_login() -> Response<String> {
        Response {
            data: None,
            message: Some("未登录".to_string()),
            code: -401,
        }
    }
    pub fn bad_request(message: &str) -> Response<String> {
        Response {
            data: None,
            message: Some(message.to_string()),
            code: -400,
        }
    }
    pub fn forbidden() -> Response<String> {
        Response {
            data: None,
            message: Some("无权操作!".to_string()),
            code: -403,
        }
    }
}

impl<T> Response<T> { 

    pub fn data(data: T) -> Response<T> {
        Response {
            data: Some(data),
            message: Some("success".to_string()),
            code: 0,
        }
    }

}