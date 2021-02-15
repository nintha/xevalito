use actix_web::*;
use crate::common::{BusinessError};
use crate::common::db::RbaitsService;
use crate::autowired::{Component, Autowired};
use rbatis::crud::CRUD;
use crate::user::entity::{User, UserSave};
use rbatis_core::types::Uuid;
use chrono::NaiveDateTime;
use rbatis_core::value::DateTimeNow;

#[derive(Default)]
pub struct UserService {
    rb: Autowired<RbaitsService>,
}

impl Component for UserService {}

impl UserService {
    pub async fn get_by_id(&self, id: &String) -> Result<Option<User>, BusinessError> {
        let user: Option<User> = self.rb.fetch_by_id("", id).await?;
        Ok(user)
    }

    pub async fn get_by_username(&self, username: &str) -> Result<Option<User>, BusinessError> {
        let wrapper = self.rb.new_wrapper().eq("username", username).check()?;
        let user: Option<User> = self.rb.fetch_by_wrapper("", &wrapper).await?;
        Ok(user)
    }

    /// if pass, return user
    pub async fn login(&self, username: &str, password: &str) -> Result<User, BusinessError> {
        let user_option = self.get_by_username(username).await?;
        let user = user_option.ok_or(BusinessError::LoginError)?;

        let pwd = &user.password;
        if !pwd.is_empty() && bcrypt::verify(password, &pwd)? {
            Ok(user)
        } else {
            Err(BusinessError::LoginError)
        }
    }

    pub async fn save_user(&self, user_save: UserSave) -> Result<String, BusinessError> {
        let mut user = User {
            id: Uuid::new_v4().to_simple().to_string(),
            username: user_save.username,
            password: user_save.password,
            create_time: NaiveDateTime::now(),
        };
        log::info!("[save_user] user={:?}", &user);

        if self.get_by_username(&user.username).await?.is_none() {
            return Err(BusinessError::ValidationError { field: "username existed".into() });
        }

        // 对密码进行hash
        let hashed_pwd = bcrypt::hash(user.password, 6);
        user.password = hashed_pwd.map_err(|e| BusinessError::InternalError { source: anyhow!(e) })?;

        self.rb.save("", &user).await?;

        Ok(user.id)
    }

    /// 获取当前登录用户ID
    pub fn current_user_id(&self) -> Option<String> {
        // TODO
        None
    }
}






