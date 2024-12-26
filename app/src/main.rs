use frame::{singleton};
use frame_support::{get_instance_by_key,get_instance_by_type, get_type_name};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use lazy_static::lazy_static;
 
#[singleton]
struct MyStruct1 {
    field: i32,
}
 
#[singleton]
struct MyStruct2 {
    field: i32,
}

fn test001() -> Option<String> {

    if let Some(instance1) = get_instance_by_type::<MyStruct1>() {
        let mut instance1 = instance1.read().unwrap();  
        println!("MyStruct1 field after modification: {}", instance1.field);
    }

    if let Some(instance1) = get_instance_by_type::<MyStruct1>() {
        let mut instance1 = instance1.write().unwrap(); // 获取可变引用
        instance1.field = 42; // 修改 instance1 的 field 字段
        println!("MyStruct1 field after modification: {}", instance1.field);
    }

    if let Some(instance1) = get_instance_by_type::<MyStruct1>() {
        let mut instance1 = instance1.read().unwrap();  
        println!("MyStruct1 field after modification: {}", instance1.field);
    }

    println!("StructName: {}", get_type_name::<MyStruct1>());
    None
}

fn main() {
   if let Some(str) = test001() {
        println!("{}", str);
   }
}