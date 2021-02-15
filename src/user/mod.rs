mod service;
mod entity;

use actix_web::*;
use crate::common::{RespResult, Resp};
use crate::user::entity::{UserSave};
pub use service::UserService;
use crate::autowired::{Autowired};
use jsonwebtoken::{Header, EncodingKey};
use crate::common::middleware::jwt::Claims;
use chrono::{Local, Duration};
use std::ops::Add;

const USER_SERVICE: Autowired<UserService> = Autowired::new();

#[post("login")]
pub async fn login(user: web::Json<UserSave>) -> RespResult {
    let user_auth: UserSave = user.into_inner();
    let login_user = USER_SERVICE.login(&user_auth.username, &user_auth.password).await?;

    let my_claims = Claims {
        sub: login_user.id,
        exp: Local::now().add(Duration::seconds(3600)).timestamp() as u64,
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("secret".as_ref()),
    ).map_err(|x| anyhow::anyhow!(x))?;

    Resp::ok(&token).to_json_result()
}

#[post("")]
pub async fn save_user(user: web::Json<UserSave>) -> RespResult {
    let user_id = USER_SERVICE.save_user(user.into_inner()).await?;
    Resp::ok(&user_id).to_json_result()
}


#[get("current")]
pub async fn current_user(req: HttpRequest) -> RespResult {
    let user_id = req.headers().get("x-claims-sub")
        .map(|v| v.to_str().unwrap_or_default())
        .filter(|x| !x.is_empty())
        .unwrap_or_default();

    let user = USER_SERVICE.get_by_id(&user_id.to_string()).await?
        .map(|mut x|{
            x.password = Default::default();
            x
        });
    Resp::ok(&user).to_json_result()
}

pub fn routing() -> Scope {
    web::scope("/users")
        .service(save_user)
        .service(current_user)
}

