pub mod proto;
use sqlx::{MySql, Pool};
use tonic::{async_trait, Request, Response, Status, Code};

use crate::{cache::Redis, db::Db};

#[derive(Debug)]
pub struct TemplateService {
    redis: Redis,
    db: Db,
}

impl TemplateService {
    pub fn new(pool: Pool<MySql>, redis: redis::Client) -> TemplateService {
        Self { db: Db::new(pool), redis: Redis::new(redis) }
    }
}

// #[async_trait]
// impl Account for TemplateService {
//     async fn get_session(&self, request: Request<AccessKey>) -> Result<Response<GetSessionReply>, Status> {
//         let r = request.into_inner();
//         if r.access_key.len()<1 {
//             return Err(Status::new(Code::InvalidArgument, "access_key不合法"))
//         }
//         match self.redis.get_session(&r.access_key).await {
//             Some(s) => {
//                 Ok(Response::new(GetSessionReply {uid: s.uid, family_id: s.family_id.unwrap_or(-1), mobile: s.mobile}))
//             },
//             None => Err(Status::new(Code::NotFound, "未登录"))
//         }
//     }
// }

