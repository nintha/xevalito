use actix_web::*;
use crate::common::db::RbaitsService;
use autowired::Component;

mod common;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    common::init_logger();
    autowired::register(RbaitsService::new_instance()?);

    let binding_address = "0.0.0.0:17000";
    HttpServer::new(|| {
        App::new()
    }).bind(binding_address)
        .expect(&format!("Can not bind to {}", binding_address))
        .run()
        .await?;
    Ok(())
}
