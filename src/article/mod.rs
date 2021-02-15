use actix_web::{delete, get, HttpRequest, post, put, Scope};
use actix_web::web;
use rbatis::plugin::page::PageRequest;

use crate::article::entity::ArticleLite;
use crate::article::service::ArticleService;
use crate::autowired::Autowired;
use crate::common::{Resp, RespResult};

mod entity;
mod service;

const ARTICLE_SERVICE: Autowired<ArticleService> = Autowired::new();

#[get("")]
pub async fn list_page(req: HttpRequest) -> RespResult {
    let q_string = qstring::QString::from(req.query_string());
    let page = q_string.get("page").unwrap_or_default().parse().unwrap_or(1);
    let size = q_string.get("size").unwrap_or_default().parse().unwrap_or(10);
    let user_id = ARTICLE_SERVICE.list_page(PageRequest::new(page, size)).await?;
    Resp::ok(&user_id).to_json_result()
}

#[post("")]
pub async fn save(article: web::Json<ArticleLite>) -> RespResult {
    let user_id = ARTICLE_SERVICE.save(article.into_inner()).await?;
    Resp::ok(&user_id).to_json_result()
}

#[put("{id}")]
pub async fn update(id: web::Path<String>,article: web::Json<ArticleLite>) -> RespResult {
    ARTICLE_SERVICE.update(id.0, article.into_inner()).await?;
    Resp::ok("").to_json_result()
}

#[delete("{id}")]
pub async fn remove(id: web::Path<String>) -> RespResult {
    ARTICLE_SERVICE.remove(&id.0).await?;
    Resp::ok("").to_json_result()
}

pub fn routing() -> Scope {
    web::scope("/articles")
        .service(list_page)
        .service(save)
        .service(update)
        .service(remove)
}

