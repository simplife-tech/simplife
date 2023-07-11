use akasha::dto::response::Response;
use axum::{extract::{State, Query}, Json, response::IntoResponse};

use crate::{app_state::AppState, dto::agenda::{AddAgendaReq, DeleteAgendaReq, GetAgendaReq, UpdateAgendaReq}, strings::{NO_FAMILY, NO_AGENDA}};

pub async fn add_agenda(
    State(state): State<AppState>,
    Json(arg): Json<AddAgendaReq>
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
        match state.db.add_agenda(&uid, &family_id, &arg.title, &arg.content).await {
            Ok(_) => {
                let _ = state.redis.remove_agenda(&family_id).await;
                Json(Response::success()).into_response()
            },
            Err(_) => {
                Json(Response::network_error()).into_response()
            }
        }
    }
}

pub async fn delete_agenda(
    State(state): State<AppState>,
    Json(arg): Json<DeleteAgendaReq>
) -> axum::response::Response {
    let agenda = match state.db.get_agenda(&arg.id).await {
        Ok(agenda) => {
            match agenda {
                Some(agenda) => agenda,
                None => return Json(Response::fail(-500, NO_AGENDA)).into_response()
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
        if agenda.family_id != family_id {
            return Json(Response::forbidden()).into_response()
        }
        match state.db.delete_agenda(&arg.id, &uid).await {
            Ok(_) => {
                let _ = state.redis.remove_agenda(&family_id).await;
                return Json(Response::success()).into_response()
            },
            Err(_) => return Json(Response::network_error()).into_response()
        }
    }
}

pub async fn agenda_list(
    State(state): State<AppState>,
    Query(arg): Query<GetAgendaReq>
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
        let agendas = match state.redis.get_agenda(&family_id, &arg.date_start, &arg.date_end, &arg.pn, &arg.ps).await {
            Ok(agendas) => agendas,
            Err(_) => None
        };
        if let Some(agendas) = agendas {
            return Json(Response::data(agendas)).into_response()
        }
        match state.db.get_agenda_list(&family_id, &akasha::time::timestamp_to_datetime(arg.date_start), &akasha::time::timestamp_to_datetime(arg.date_end), &arg.pn, &arg.ps).await {
            Ok(ledgers) => {
                let _ = state.redis.set_agenda(&family_id, &ledgers, &arg.date_start, &arg.date_end, &arg.pn, &arg.ps).await;
                return Json(Response::data(agendas)).into_response()
            },
            Err(_) => {
                return Json(Response::network_error()).into_response()
            }
        }
    } 
}

pub async fn update_agenda(
    State(state): State<AppState>,
    Query(arg): Query<UpdateAgendaReq>
) -> axum::response::Response {
    let agenda = match state.db.get_agenda(&arg.id).await {
        Ok(agenda) => {
            match agenda {
                Some(agenda) => agenda,
                None => return Json(Response::fail(-500, NO_AGENDA)).into_response()
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
        if agenda.family_id != family_id {
            return Json(Response::forbidden()).into_response()
        }
        match state.db.update_agenda(&arg.id, &uid, &arg.title, &arg.content).await {
            Ok(_) => {
                let _ = state.redis.remove_agenda(&family_id).await;
                return Json(Response::success()).into_response()
            },
            Err(_) => return Json(Response::network_error()).into_response()
        }
    }
}