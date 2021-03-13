use async_std::sync::Arc;
use dashmap::DashMap;
use std::any::Any;
use std::any::type_name;
use std::ops::Deref;
use once_cell::sync::OnceCell;
use std::sync::Mutex;
use chrono::Local;
use std::fmt::{Debug, Formatter};
use anyhow::Error;
use std::future::Future;
use std::pin::Pin;

fn component_mutex() -> &'static Mutex<i64> {
    static INSTANCE: OnceCell<Mutex<i64>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

fn component_dashmap() -> &'static DashMap<String, Arc<dyn Any + 'static + Send + Sync>> {
    static INSTANCE: OnceCell<DashMap<String, Arc<dyn Any + 'static + Send + Sync>>> = OnceCell::new();
    INSTANCE.get_or_init(Default::default)
}

fn get_component<T: Component>() -> Option<Arc<T>> {
    component_dashmap().get(type_name::<T>())
        .map(|x| x.value().clone())
        .map(|x| x.downcast::<T>().ok())
        .flatten()
}

/// return true if component exists
pub fn exist_component<T: Component>() -> bool {
    component_dashmap().contains_key(type_name::<T>())
}

pub trait Component: Any + 'static + Send + Sync {
    /// create a new component instance
    fn new_instance() -> Pin<Box<dyn Future<Output=Result<Arc<Self>, anyhow::Error>>>>;

    fn register() where Self: std::marker::Sized {
        let name = type_name::<Self>();
        // 在注册组件的时候进行加锁，防止出现多次初始化
        if let Ok(mut timestamp) = component_mutex().lock() {
            if component_dashmap().contains_key(name) {
                return;
            }

            let component: Arc<dyn Any + 'static + Send + Sync> = match async_std::task::block_on(Self::new_instance()) {
                Ok(v) => v,
                Err(e) => {
                    log::warn!("[Component] register failure, {}", e);
                    return;
                }
            };
            component_dashmap().insert(name.to_string(), component);
            log::info!("[Component] register, name={}", name);
            *timestamp = Local::now().timestamp_millis();
        }
    }

    fn check_health(&self) -> bool {
        false
    }
}

/// lazy autowired
pub struct Autowired<T> {
    inner: OnceCell<Arc<T>>,
}

impl<T> Autowired<T> {
    pub const fn new() -> Self {
        Autowired { inner: OnceCell::new() }
    }
}

impl<T: Component> Deref for Autowired<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        self.inner.get_or_init(|| {
            if !exist_component::<T>() {
                T::register()
            }
            get_component::<T>().unwrap_or_else(||
                panic!(format!("[Autowired] not found component {}", type_name::<T>()))
            )
        })
    }
}

impl<T: Component> Default for Autowired<T> {
    fn default() -> Self {
        Autowired::new()
    }
}

