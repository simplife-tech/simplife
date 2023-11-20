use akasha::error::Error;
pub(crate) 
use akasha::{dto::response::Response, Context, db::Db, redis::Redis};
use axum::{Json, Extension, async_trait};

use crate::{dto::login::{LoginReq, LoginReply}, service::UserService};

#[derive(Clone)]
pub struct LoginHandler {
    pub user_service: UserService
}

#[async_trait]
pub trait LoginHandlerTrait: Send + Clone + Sync + 'static {
    async fn user_login(&self, ctx: &Context, req: LoginReq) -> Result<LoginReply, Error>;
}

impl LoginHandler {
    pub fn new(db: Db, redis: Redis) -> LoginHandler {
        let user_service = UserService::new(db, redis);
        Self { user_service }
    }
}

#[async_trait]
impl LoginHandlerTrait for LoginHandler {
    async fn user_login(&self, ctx: &Context, req: LoginReq) -> Result<LoginReply, Error> {
        match self.user_service.user_login(ctx, req).await {
            Ok(login_reply) => Ok(login_reply),
            Err(err) => Err(err)
        }
    }
}

pub async fn user_login<T: LoginHandlerTrait>(
    Extension(handler): Extension<T>,
    Extension(ctx): Extension<Context>,
    Json(req): Json<LoginReq>
) -> Result<Response<LoginReply>, Error> {
    match handler.user_login(&ctx, req).await {
        Ok(login_reply) => Ok(Response::data(login_reply)),
        Err(err) => Err(err)
    }
}