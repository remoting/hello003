use frame::spring::traits::Post; 
use log::info;  
use std::any::{self, Any};

#[derive(Debug)]
struct MyStruct1 {
    field: i32, 
}

impl Post for MyStruct1 {
    fn post_construct(&self) {
        info!("MyStruct1 post_construct called with field: {}", self.field);
    }
}
 
impl MyStruct1 {
    pub fn new() -> MyStruct1 {
        MyStruct1 {
            field: 100,
        }
    }
    pub fn as_any(&self) -> &dyn Any {
        self
    } 
} 

fn test002(obj: &dyn Post) -> Option<String> {
    obj.post_construct();
    Some("Completed".to_string())  
}
 
fn is_impl_trait<T: Any + ?Sized>(_: &T) -> bool {
    true
}

fn main() {
    log4rs::init_file("./app2/log4rs.yaml", Default::default()).unwrap();

    let instance = MyStruct1::new(); 
    let x = is_impl_trait::<dyn Post>(&instance);
    
    info!("isimpl-{:?}", x);
    test002(&instance); 
}