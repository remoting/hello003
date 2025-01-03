use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use lazy_static::lazy_static;
use std::any::Any;
use std::any::type_name;

pub mod traits;


lazy_static! {
    static ref INSTANCE_MAP: RwLock<HashMap<String, Box<dyn Any + Send + Sync>>> = RwLock::new(HashMap::new());
    static ref HANDLER_MAP: Arc<RwLock<HashMap<&'static str, fn()>>> = Arc::new(RwLock::new(HashMap::new()));
}

pub fn get_instance_by_type<T: 'static + Send + Sync>() -> Option<std::sync::Arc<RwLock<T>>> {
    let key = type_name::<T>().to_string();
    return get_instance_by_key(&key);
}

pub fn get_instance_by_key<T: 'static + Send + Sync>(key: &str) -> Option<std::sync::Arc<RwLock<T>>> {
    let map = INSTANCE_MAP.read().ok()?;
    map.get(key).and_then(|boxed| boxed.downcast_ref::<std::sync::Arc<RwLock<T>>>().cloned())
}

pub fn register_instance<T: 'static + Send + Sync>(key: &str, instance: std::sync::Arc<RwLock<T>>) {
    if let Ok(mut map) = INSTANCE_MAP.write() {
        map.insert(key.to_string(), Box::new(instance));
    } 
}

pub fn register_instance_by_type<T: 'static + Send + Sync>(instance: std::sync::Arc<RwLock<T>>) {
    let key = get_type_name::<T>();
    register_instance(&key, instance); 
}

pub fn get_type_name<T: 'static + Send + Sync>() -> String {
    type_name::<T>().to_string()
}

pub fn register_function(name: &'static str, func: fn()) {
    let mut map = HANDLER_MAP.write().unwrap();
    map.insert(name, func);
}

pub fn get_function(name: &str) -> Option<fn()> {
    let map = HANDLER_MAP.read().unwrap();
    map.get(name).cloned()
}