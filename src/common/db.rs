use rbatis::rbatis::Rbatis;

use std::ops::Deref;
use std::time::Duration;
use std::sync::Arc;
use rbatis::core::db::DBPoolOptions;
use async_std::task;
use autowired::Component;
use std::error::Error;


pub struct RbaitsService(Rbatis);

impl Deref for RbaitsService {
    type Target = Rbatis;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RbaitsService {
    const URL: &'static str = "mysql://root:123456@localhost:3306/test?serverTimezone=Asia/Shanghai";
}

impl Component for RbaitsService {
    type Error = rbatis::Error;

    fn new_instance() -> Result<Arc<Self>, Self::Error> {
        let rbatis = RbaitsService(Rbatis::new());
        let mut opt = DBPoolOptions::new();
        opt.max_connections = num_cpus::get() as u32 + 1;
        opt.connect_timeout = Duration::from_secs(3);
        task::block_on(rbatis.link_opt(Self::URL, &opt))?;
        Ok(Arc::new(rbatis))
    }
}

