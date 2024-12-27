use frame::{singleton, register_command,spring};
use frame_support::{get_instance_by_key,get_instance_by_type, get_type_name, get_function};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use lazy_static::lazy_static;
use log::{info, warn};  

#[derive(spring)]
struct MyStruct1 {
    field: i32,
}

impl MyStruct1 {
    //#[register_command]
    pub fn test1(&self) -> String {
        format!("MyStruct1 {{ field: {} }}", self.field)
    }

    //#[register_command]
    pub fn test2(&self) -> String {
        format!("MyStruct1 {{ field: {} }}", self.field)
    }
}

//#[singleton]
struct MyStruct2 {
    field: i32,
}

// fn test001() -> Option<String> {
//     if let Some(instance1) = get_instance_by_type::<MyStruct1>() {
//         let mut instance1 = instance1.read().unwrap();  
//         info!("MyStruct1 field after modification: {}", instance1.field); // 使用日志记录
//     }

//     if let Some(instance1) = get_instance_by_type::<MyStruct1>() {
//         let mut instance1 = instance1.write().unwrap(); // 获取可变引用
//         instance1.field = 42; // 修改 instance1 的 field 字段
//         info!("MyStruct1 field after modification: {}", instance1.field); // 使用日志记录
//     }

//     if let Some(instance1) = get_instance_by_type::<MyStruct1>() {
//         let instance1 = instance1.read().ok()?;  
//         info!("MyStruct1 field after modification: {}", instance1.field); // 使用日志记录
//         info!("MyStruct1 field after modification: {}", instance1.test1()); // 使用日志记录
//     }

//     info!("StructName: {}", get_type_name::<MyStruct1>()); // 使用日志记录
//     None
// }

fn main() {
    log4rs::init_file("./app/log4rs.yaml", Default::default()).unwrap();
    // env_logger::init(); // 初始化日志记录
    
    let instance = MyStruct1 { field: 42 };
    info!("ss{}www", instance.spring_method());
    info!("MyStruct1 instance created with field: {}", instance.field);
 
    // 打印注册的 handler
    // let handler_map = frame_support::get_handler_map().read().unwrap();
    // for (name, _) in handler_map.iter() {
    //     println!("Registered handler: {}", name);
    // }

    // if let Some(str) = test001() {
    //     println!("{}", str);
    // }
}