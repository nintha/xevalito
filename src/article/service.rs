use rbatis::crud::CRUD;

use crate::article::entity::{Article, ArticleLite};
use crate::autowired::{Autowired, Component};
use crate::common::BusinessResult;
use crate::common::db::RbaitsService;
use crate::user::UserService;
use rbatis::plugin::page::{PageRequest, Page};
use rbatis_core::types::Uuid;
use chrono::NaiveDateTime;
use rbatis_core::value::DateTimeNow;

#[derive(Default)]
pub struct ArticleService {
    rb: Autowired<RbaitsService>,
    user_service: Autowired<UserService>,
}

impl Component for ArticleService {}

impl ArticleService {
    pub async fn list_page(&self, req: PageRequest) -> BusinessResult<Page<Article>> {
        let wrapper = self.rb.new_wrapper().check()?;
        let list: Page<Article> = self.rb.fetch_page_by_wrapper("", &wrapper, &req).await?;
        Ok(list)
    }

    pub async fn save(&self, article_lite: ArticleLite) -> BusinessResult<String> {
        let article = Article {
            id: Uuid::new_v4().to_simple().to_string(),
            title: article_lite.title,
            content: article_lite.content,
            creator_id: self.user_service.current_user_id().unwrap_or_default(),
            create_time: NaiveDateTime::now(),
        };

        self.rb.save("", &article).await?;
        Ok(article.id)
    }

    pub async fn update(&self, id: String, article_lite: ArticleLite) -> BusinessResult<()> {
        let mut source: Article = self.rb.fetch_by_id("", &id).await?;
        source.title = article_lite.title;
        source.content = article_lite.content;
        self.rb.update_by_id("", &source).await?;
        Ok(())
    }

    pub async fn remove(&self, id: &str) -> BusinessResult<()> {
        self.rb.remove_by_id::<Article>("", &id.to_string()).await?;
        Ok(())
    }
}

