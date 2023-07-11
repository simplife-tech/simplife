use redis::{AsyncCommands, RedisError};

use crate::db::agenda::Agenda;

use super::Redis;

pub fn _family_agenda_key(family_id: &i64) -> String {
    format!("AGENDA:FAMILY:{}", family_id.to_string())
}

pub fn _agenda_hset_field(date_start: &i64, date_end: &i64, pn: &i64, ps: &i64) -> String {
    format!("DATE_{}_{}_PN_{}_PS_{}", date_start.to_string(), date_end.to_string(), pn.to_string(), ps.to_string())
}

impl Redis {

    pub async fn set_agenda(&self, family_id: &i64, agendas: &Vec<Agenda>, date_start: &i64, date_end: &i64, pn: &i64, ps: &i64) -> Result<(), RedisError> {
        let mut manager = self.manager.clone();
        let key = _family_agenda_key(&family_id);
        let field = _agenda_hset_field(date_start, date_end, pn, ps);
        match manager.hset::<_, _, _, ()>(&key, field, serde_json::to_string(agendas).unwrap()).await {
            Ok(_) => {
                match manager.expire::<_, ()>(&key, 60*60*1).await {
                    Ok(_) => Ok(()),
                    Err(err) => {
                        log::error!("redis error! {}", err);
                        return Err(err)
                    }
                }
            },
            Err(err) => {
                log::error!("redis error! {}", err);
                return Err(err)
            }
        }
    }

    pub async fn get_agenda(&self, family_id: &i64, date_start: &i64, date_end: &i64, pn: &i64, ps: &i64) -> Result<Option<Vec<Agenda>>, RedisError> {
        let mut manager = self.manager.clone();
        let key = _family_agenda_key(&family_id);
        let field = _agenda_hset_field(date_start, date_end, pn, ps);
        let exist: i8 = match manager.hexists(&key, &field).await {
            Ok(exist) => exist,
            Err(err) => {
                log::error!("redis error! {}", err);
                return Err(err)
            }
        };
        if exist == 1 {
            let s: String = match manager.hget(&key, &field).await {
                Ok(s) => s,
                Err(err) => {
                    log::error!("redis error! {}", err);
                    return Err(err)
                }
            };
            let agendas: Vec<Agenda> = serde_json::from_str(&s).unwrap_or(vec![]);
            return Ok(Some(agendas))
        } else {
            return Ok(None)
        }
    }

    pub async fn remove_agenda(&self, family_id: &i64) -> Result<(), RedisError> {
        let mut manager = self.manager.clone();
        let key = _family_agenda_key(&family_id);
        match manager.del::<_, ()>(key).await {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("redis error! {}", err);
                Err(err)
            }
        }
    }
}