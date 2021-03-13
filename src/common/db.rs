use rbatis::rbatis::Rbatis;

use crate::autowired::Component;
use std::ops::Deref;
use std::time::Duration;
use std::sync::Arc;
use anyhow::Error;
use rbatis::core::db::DBPoolOptions;
use async_std::task;
use std::future::Future;
use std::pin::Pin;
use futures_util::FutureExt;


pub struct RbaitsService(Rbatis);

impl Deref for RbaitsService {
    type Target = Rbatis;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RbaitsService {
    const MYSQL_URL: &'static str =
        "mysql://root:123456@localhost:3306/test?serverTimezone=Asia/Shanghai";
}

impl Component for RbaitsService {
    fn new_instance() -> Pin<Box<dyn Future<Output=Result<Arc<Self>, anyhow::Error>>>> {
        let fut = async {
            let rbatis = RbaitsService(Rbatis::new());
            let mut opt = DBPoolOptions::new();
            opt.max_connections = num_cpus::get() as u32 + 1;
            opt.connect_timeout = Duration::from_secs(3);
            if let Err(e) = rbatis.link_opt(Self::MYSQL_URL, &opt).await {
                Err(anyhow::anyhow!(e))
            } else {
                Ok(Arc::new(rbatis))
            }
        };
        Box::pin(fut)
    }

    fn check_health(&self) -> bool {
        let future = self.fetch("", "select 1;");
        let result: i32 = task::block_on(future).unwrap_or_default();
        result == 1
    }
}

