

trait MyTrait {
    fn my_trait_method(&self) -> i32;
}
struct MyStruct {
    a: i32,
    b: i32,
}
struct MyStruct2 {
    a: i32,
    b: i32,
}

impl MyTrait for MyStruct {
    fn my_trait_method(&self) -> i32 {
        self.a + self.b
    } 
}

fn main() {
    let my_struct = MyStruct { a: 1, b: 2 };
    let my_trait: &dyn MyTrait = &my_struct;
    println!("my_trait_method: {}", my_trait.my_trait_method());

    let my_struct2 = MyStruct2 { a: 1, b: 2 };
    let x: &dyn MyTrait = &my_struct2;
    println!("my_trait_method: {}", x.my_trait_method());
}