use rbatis::rbatis::Rbatis;
use rbatis_core::db::PoolOptions;

use crate::autowired::Component;
use std::ops::Deref;
use std::time::Duration;

pub struct RbaitsService(Rbatis);

impl Deref for RbaitsService {
    type Target = Rbatis;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RbaitsService {
    const MYSQL_URL: &'static str =
        "mysql://root:123456@localhost:3306/bbs_rs?serverTimezone=Asia/Shanghai";
}

impl Component for RbaitsService {}

impl Default for RbaitsService {
    fn default() -> Self {
        let rbatis = Rbatis::new();
        let mut opt = PoolOptions::new();
        opt.max_size = num_cpus::get() as u32;
        opt.connect_timeout = Duration::from_secs(5);
        async_std::task::block_on(async {
            rbatis.link_opt(Self::MYSQL_URL, &opt).await.unwrap();
        });
        RbaitsService(rbatis)
    }
}
