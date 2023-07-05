use akasha::dto::response::Response;
use axum::{extract::State, Json, response::IntoResponse};

use crate::{app_state::AppState, dto::ledger::LedgerReq};

pub async fn add_ledger(
    State(state): State<AppState>,
    Json(arg): Json<LedgerReq>
) -> axum::response::Response {
    let uid = match state.grpc_client.get_uid(&arg.access_key).await {
        Ok(uid) => uid,
        Err(_) => return Json(Response::not_login()).into_response(),
    };
    let family_id = match state.grpc_client.get_family_id(&uid).await {
        Ok(family_id) => family_id,
        Err(_) => return Json(Response::network_error()).into_response(),
    };
    if family_id <= 0 {
        match state.db.add_ledger_with_uid(&uid, &akasha::time::timestamp_to_datetime(arg.date), &arg.ammount, &arg.comment).await {
            Ok(_) => {
                Json(Response::success()).into_response()
            },
            Err(_) => {
                Json(Response::network_error()).into_response()
            }
        }
    } else {
        match state.db.add_ledger_with_uid_and_family_id(&uid, &family_id, &akasha::time::timestamp_to_datetime(arg.date), &arg.ammount, &arg.comment).await {
            Ok(_) => {
                Json(Response::success()).into_response()
            },
            Err(_) => {
                Json(Response::network_error()).into_response()
            }
        }
    }
}