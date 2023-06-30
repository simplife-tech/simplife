
use akasha::dto::response::Response;
use axum::{Json, response::IntoResponse, extract::State};

use crate::{dto::login::{LoginReq, LoginRsp}, app_state::AppState};

pub async fn user_login(
    State(state): State<AppState>,
    Json(arg): Json<LoginReq>
) -> axum::response::Response {
    match state.db.find_by_mobile_and_password(&arg.mobile, &arg.password).await {
        Ok(user) => {
            let session_id = state.redis.set_session(&user.id).await;
            Json(Response::data(LoginRsp{
                uid: user.id,
                access_key: session_id
            })).into_response()
        },
        Err(msg) => {
            Json(Response::fail(-500, msg)).into_response()
        }
    }
}
