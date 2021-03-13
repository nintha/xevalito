use actix_web::*;
use crate::autowired::{Autowired, Component};
use crate::common::db::RbaitsService;

mod autowired;
mod common;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    common::init_logger();
    dbg!(Autowired::<RbaitsService>::new().check_health());

    let binding_address = "0.0.0.0:17000";
    HttpServer::new(|| {
        App::new()
    }).bind(binding_address)
        .expect(&format!("Can not bind to {}", binding_address))
        .run()
        .await
}
