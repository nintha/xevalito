#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate rbatis_macro_driver;

use actix_web::*;

use crate::common::*;

mod article;
mod autowired;
mod common;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();

    let binding_address = "localhost:7000";

    HttpServer::new(|| {
        App::new()
            .wrap(common::middleware::jwt::JwtPlugin)
            .service(user::login)
            .service(user::routing())
            .service(article::routing())
    })
        .bind(binding_address)
        .expect(&format!("Can not bind to {}", binding_address))
        .run()
        .await
}
