use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub data: Option<T>,
    pub message: Option<String>,
    pub code: i32,
}

impl Response<String> {
    pub fn success() -> Response<String> {
        Response {
            data: None,
            message: Some("success".to_string()),
            code: 0,
        }
    }
    pub fn fail(code: i32, message: String) -> Response<String> {
        Response {
            data: None,
            message: Some(message.to_string()),
            code,
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