use akasha::error::Error;

pub const LOGIN_FAILED: Error = Error::ServiceError(-401, "用户名或密码错误");
pub const NOT_LOGIN: Error = Error::ServiceError(-401, "用户未登录");

