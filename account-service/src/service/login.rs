
use akasha::{dto::response::Response, opentelemetry::trace::{TraceId, SpanId}};
use axum::{Json, response::IntoResponse, extract::State, Extension};

use crate::{dto::login::{LoginReq, LoginRsp}, app_state::AppState};
use akasha::crypto::sha3_512;

pub async fn user_login(
    Extension(oc): Extension<akasha::opentelemetry::Context>,
    State(state): State<AppState>,
    Json(arg): Json<LoginReq>
) -> axum::response::Response {
    let password_hash = sha3_512(arg.password.to_string());
    match state.db.find_user_by_mobile(&oc, &arg.mobile).await {
        Ok(user) => {
            if user.password == password_hash {
                match state.redis.set_session(&oc, &user.id).await {
                    Ok(session_id) => {
                        Json(Response::data(LoginRsp{
                            uid: user.id,
                            access_key: session_id
                        })).into_response()
                    },
                    Err(_) => {
                        Json(Response::network_error()).into_response()
                    }
                }
            } else {
                Json(Response::login_error()).into_response()
            }
        },
        Err(_) => {
            Json(Response::login_error()).into_response()
        }
    }
}
