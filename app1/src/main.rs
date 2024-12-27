use frame::spring::traits::Post;
use frame_macros::{singleton, hello,command,controller};
use frame::spring::{get_instance_by_key,get_instance_by_type, get_type_name, get_function};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use lazy_static::lazy_static;
use log::{info, warn};  
use std::any::Any;
#[singleton]
#[derive(hello,Debug)]
struct MyStruct1 {
    field: i32, 
}

impl frame::spring::traits::Post for MyStruct1 {
    fn post_construct(&self) {
        info!("MyStruct1 post_construct called with field: {}", self.field);
    }
}

//#[controller(command)]
impl MyStruct1 {

    pub fn as_any(&self) -> &dyn Any {
        info!("MyStruct1 post_construct called with field: {}", self.field);
        self
    }
    //#[command]
    pub fn test1(&self,args:String) -> String {
        format!("MyStruct1 {{ field: {} }}", self.field)
    }
 
   // #[command]
    pub fn test2(&self,args:String) -> String {
        format!("MyStruct1 {{ field: {} }}", self.field)
    }
}
// #[controller(command)]
// impl MyStruct1 {
//     #[command]
//     pub fn test3(&self) -> String {
//         format!("hello method called with field: {}", self.field)
//     } 
// }

//#[singleton]
// struct MyStruct2 {
//     field: i32,
// }

// fn test001() -> Option<String> {
//     if let Some(instance1) = get_instance_by_type::<MyStruct1>() {
//         let mut instance1 = instance1.read().unwrap();  
//         info!("MyStruct1 field after modification: {}", instance1.field);  
//     }

//     if let Some(instance1) = get_instance_by_type::<MyStruct1>() {
//         let mut instance1 = instance1.write().unwrap();  
//         instance1.field = 42;  
//         info!("MyStruct1 field after modification: {}", instance1.field);  
//     }

//     if let Some(instance1) = get_instance_by_type::<MyStruct1>() {
//         let instance1 = instance1.read().ok()?;  
//         info!("MyStruct1 field after modification: {}", instance1.field);  
//         let method_name = "test1".to_string();
//         // 这个地方我已经获取到了 instance1 的引用，并且我也知道了它有个方法method_name,我现在要调用这个方法应该怎么写

//         //info!("MyStruct1 field after modification: {}", instance1.test1());  
//     }

//     info!("StructName: {}", get_type_name::<MyStruct1>());  


//     if let Some(instance2) = get_instance_by_type::<MyStruct2>() {
//         let instance2 = instance2.read().ok()?;  

//         info!("--{}-", instance2.field);  
//     }

//     None
// }

   // instance.methods.insert("test1".to_string(), MyStruct1::test1);
    // if let Some(func) = instance.get_method("test1") {
    //     let r = func(&instance,"".to_string()); 
    //     info!("--{}--", r); 
    // }

fn test002() ->Option<String> {
    let instance = MyStruct1::new(); 
    let mut xxx = instance.as_any();
    // if let Some(post_instance) = xxx.downcast_ref::<dyn Post>() {
    //     post_instance.post_construct();
    //     info!("--{}--", "true");
    // }else{
    //     info!("--{}--", "false");
    // }
    Some("Completed".to_string()) // 返回一个 Option<String>
}

fn main() {
    log4rs::init_file("./log4rs.yaml", Default::default()).unwrap();
    //log4rs::init_file("./log4rs.yaml", Default::default()).unwrap();
    // env_logger::init(); // 初始化日志记录
    
    test002();
    //test001();
 
    // 打印注册的 handler
    // let handler_map = frame_support::get_handler_map().read().unwrap();
    // for (name, _) in handler_map.iter() {
    //     println!("Registered handler: {}", name);
    // }

    // if let Some(str) = test001() {
    //     println!("{}", str);
    // }
}