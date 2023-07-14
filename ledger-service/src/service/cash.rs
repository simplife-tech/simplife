use akasha::dto::response::Response;
use axum::{extract::{State, Query}, Json, response::IntoResponse};

use crate::{app_state::AppState, dto::cash::{RecordCashReq, DeleteCashRecordReq, GetCashRecordReq}, strings::{NO_FAMILY, NO_CASH_RECORD}};


pub async fn record_cash(
    State(state): State<AppState>,
    Json(arg): Json<RecordCashReq>
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
        return Json(Response::fail(-500, NO_FAMILY)).into_response()    
    } else {
        match state.db.add_cash_record(&family_id, &akasha::time::timestamp_to_datetime(arg.date), &arg.ammount).await {
            Ok(_) => {
                Json(Response::success()).into_response()
            },
            Err(_) => {
                Json(Response::network_error()).into_response()
            }
        }
    }
}

pub async fn delete_cash_record(
    State(state): State<AppState>,
    Json(arg): Json<DeleteCashRecordReq>
) -> axum::response::Response {
    let cash_record = match state.db.get_cash_record(&arg.id).await {
        Ok(cash_record) => {
            match cash_record {
                Some(cash_record) => cash_record,
                None => return Json(Response::fail(-500, NO_CASH_RECORD)).into_response()
            }
        },
        Err(_) => return Json(Response::network_error()).into_response()

    };
    let uid = match state.grpc_client.get_uid(&arg.access_key).await {
        Ok(uid) => uid,
        Err(_) => return Json(Response::not_login()).into_response(),
    };
    let family_id = match state.grpc_client.get_family_id(&uid).await {
        Ok(family_id) => family_id,
        Err(_) => return Json(Response::network_error()).into_response(),
    };
    if family_id <= 0 {
        return Json(Response::fail(-500, NO_FAMILY)).into_response()    
    } else {
        if cash_record.family_id != family_id {
            return Json(Response::forbidden()).into_response()
        }
        match state.db.delete_cash_record(&arg.id).await {
            Ok(_) => {
                return Json(Response::success()).into_response()
            },
            Err(_) => return Json(Response::network_error()).into_response()
        }
    }
}

pub async fn cash_record_list(
    State(state): State<AppState>,
    Query(arg): Query<GetCashRecordReq>
) -> axum::response::Response {
    if arg.pn<=0 || arg.ps <=0 {
        return Json(Response::bad_request("参数错误")).into_response()
    }
    let uid = match state.grpc_client.get_uid(&arg.access_key).await {
        Ok(uid) => uid,
        Err(_) => return Json(Response::not_login()).into_response(),
    };
    let family_id = match state.grpc_client.get_family_id(&uid).await {
        Ok(family_id) => family_id,
        Err(_) => return Json(Response::network_error()).into_response(),
    };
    if family_id <= 0 {
        return Json(Response::fail(-500, NO_FAMILY)).into_response()    
    } else {
        match state.db.get_cash_record_list(&family_id, &akasha::time::timestamp_to_datetime(arg.date_start), &akasha::time::timestamp_to_datetime(arg.date_end), &arg.pn, &arg.ps).await {
            Ok(cash_recorders) => {
                return Json(Response::data(cash_recorders)).into_response()
            },
            Err(_) => {
                return Json(Response::network_error()).into_response()
            }
        }
    } 
}