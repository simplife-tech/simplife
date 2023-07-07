use akasha::dto::response::Response;
use axum::{extract::{State, Query}, Json, response::IntoResponse};

use crate::{app_state::AppState, dto::ledger::{AddLedgerReq, GetLedgerReq, DeleteLedgerReq}, strings::{NO_FAMILY, NO_LEDGER}};

pub async fn add_ledger(
    State(state): State<AppState>,
    Json(arg): Json<AddLedgerReq>
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
                let _ = state.redis.remove_user_ledger(&uid).await;
                Json(Response::success()).into_response()
            },
            Err(_) => {
                Json(Response::network_error()).into_response()
            }
        }
    } else {
        match state.db.add_ledger_with_uid_and_family_id(&uid, &family_id, &akasha::time::timestamp_to_datetime(arg.date), &arg.ammount, &arg.comment).await {
            Ok(_) => {
                let _ = state.redis.remove_family_ledger(&family_id).await;
                Json(Response::success()).into_response()
            },
            Err(_) => {
                Json(Response::network_error()).into_response()
            }
        }
    }
}

pub async fn delete_ledger(
    State(state): State<AppState>,
    Json(arg): Json<DeleteLedgerReq>
) -> axum::response::Response {
    let ledger = match state.db.get_ledger(&arg.id).await {
        Ok(ledger) => {
            match ledger {
                Some(ledger) => ledger,
                None => return Json(Response::fail(-500, NO_LEDGER)).into_response()
            }
        },
        Err(_) => return Json(Response::network_error()).into_response()

    };
    let uid = match state.grpc_client.get_uid(&arg.access_key).await {
        Ok(uid) => uid,
        Err(_) => return Json(Response::not_login()).into_response(),
    };
    if ledger.uid != uid {
        return Json(Response::forbidden()).into_response()
    }
    match state.db.delete_ledger(&ledger.id).await {
        Ok(_) => {
            let _ = state.redis.remove_user_ledger(&uid).await;
            if let Ok(family_id) = state.grpc_client.get_family_id(&uid).await {
                if family_id>0 {
                    let _ = state.redis.remove_family_ledger(&family_id).await;
                }
            }
            return Json(Response::success()).into_response()
        },
        Err(_) => return Json(Response::network_error()).into_response()
    }
}

pub async fn ledger_list(
    State(state): State<AppState>,
    Query(arg): Query<GetLedgerReq>
) -> axum::response::Response {
    if arg.pn<=0 || arg.ps <=0 {
        return Json(Response::bad_request("参数错误")).into_response()
    }
    let uid = match state.grpc_client.get_uid(&arg.access_key).await {
        Ok(uid) => uid,
        Err(_) => return Json(Response::not_login()).into_response(),
    };
    if arg.kind == "family" {
        let family_id = match state.grpc_client.get_family_id(&uid).await {
            Ok(family_id) => family_id,
            Err(_) => return Json(Response::network_error()).into_response(),
        };
        if family_id > 0 {
            let ledgers = match state.redis.get_family_ledger(&family_id, &arg.date_start, &arg.date_end, &arg.pn, &arg.ps).await {
                Ok(ledgers) => ledgers,
                Err(_) => None
            };
            if let Some(ledgers) = ledgers {
                return Json(Response::data(ledgers)).into_response()
            }
            match state.db.get_family_ledger_list(&family_id, &akasha::time::timestamp_to_datetime(arg.date_start), &akasha::time::timestamp_to_datetime(arg.date_end), &arg.pn, &arg.ps).await {
                Ok(ledgers) => {
                    let _ = state.redis.set_family_ledger(&family_id, &ledgers, &arg.date_start, &arg.date_end, &arg.pn, &arg.ps).await;
                    return Json(Response::data(ledgers)).into_response()
                },
                Err(_) => {
                    return Json(Response::network_error()).into_response()
                }
            }
        } else {
            return Json(Response::fail(-500, NO_FAMILY)).into_response()
        }
    } else {
        let ledgers = match state.redis.get_user_ledger(&uid, &arg.date_start, &arg.date_end, &arg.pn, &arg.ps).await {
            Ok(ledgers) => ledgers,
            Err(_) => None
        };
        if let Some(ledgers) = ledgers {
            return Json(Response::data(ledgers)).into_response()
        }
        match state.db.get_user_ledger_list(&uid, &akasha::time::timestamp_to_datetime(arg.date_start), &akasha::time::timestamp_to_datetime(arg.date_end), &arg.pn, &arg.ps).await {
            Ok(ledgers) => {
                let _ = state.redis.set_user_ledger(&uid, &ledgers, &arg.date_start, &arg.date_end, &arg.pn, &arg.ps).await;
                return Json(Response::data(ledgers)).into_response()
            },
            Err(_) => {
                return Json(Response::network_error()).into_response()
            }
        }
    }
}