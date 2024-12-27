use frame::{singleton, register_command,hello,command,controller};
use frame_support::{get_instance_by_key,get_instance_by_type, get_type_name, get_function};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use lazy_static::lazy_static;
use log::{info, warn};  

#[singleton]
#[derive(hello)]
struct MyStruct1 {
    field: i32,
}

#[controller(command)]
impl MyStruct1 {
    #[command]
    pub fn test1(&self) -> String {
        format!("MyStruct1 {{ field: {} }}", self.field)
    }
 
    #[command]
    pub fn test2(&self) -> String {
        format!("MyStruct1 {{ field: {} }}", self.field)
    }
}
#[controller(command)]
impl MyStruct1 {
    #[command]
    pub fn test3(&self) -> String {
        format!("hello method called with field: {}", self.field)
    }
    
}

#[singleton]
struct MyStruct2 {
    field: i32,
}

fn test001() -> Option<String> {
    if let Some(instance1) = get_instance_by_type::<MyStruct1>() {
        let mut instance1 = instance1.read().unwrap();  
        info!("MyStruct1 field after modification: {}", instance1.field);  
    }

    if let Some(instance1) = get_instance_by_type::<MyStruct1>() {
        let mut instance1 = instance1.write().unwrap();  
        instance1.field = 42;  
        info!("MyStruct1 field after modification: {}", instance1.field);  
    }

    if let Some(instance1) = get_instance_by_type::<MyStruct1>() {
        let instance1 = instance1.read().ok()?;  
        info!("MyStruct1 field after modification: {}", instance1.field);  
        info!("MyStruct1 field after modification: {}", instance1.test1());  
    }

    info!("StructName: {}", get_type_name::<MyStruct1>());  


    if let Some(instance2) = get_instance_by_type::<MyStruct2>() {
        let instance2 = instance2.read().ok()?;  
        info!("MyStruct2 field after modification: {}", instance2.field);  
    }

    None
}

fn main() {
    log4rs::init_file("./log4rs.yaml", Default::default()).unwrap();
    // env_logger::init(); // 初始化日志记录
    
    test001();
 
    // 打印注册的 handler
    // let handler_map = frame_support::get_handler_map().read().unwrap();
    // for (name, _) in handler_map.iter() {
    //     println!("Registered handler: {}", name);
    // }

    // if let Some(str) = test001() {
    //     println!("{}", str);
    // }
}