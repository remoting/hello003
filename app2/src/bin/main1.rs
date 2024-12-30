 
use std::any::Any;
use std::sync::Arc;
use std::collections::HashMap;

trait Interface: Any + Send + Sync {}
impl<T: Any + Send + Sync> Interface for T {}

struct Module {
    components: HashMap<String, Arc<dyn Any + Send + Sync>>,
}

impl Module {
    fn resolve_ref<T: Interface>(&self) -> &T {
        let key = std::any::type_name::<T>();
        let component = self.components
            .get("HelloWorldImpl")
            .expect("Component not found")
            .downcast_ref::<T>()
            .expect("Failed to downcast component");

        component
    }
}
// 定义一个特性
trait HelloWorld: Interface {
    fn say_hello(&self);
}

// 实现特性的结构体
struct HelloWorldImpl;

impl HelloWorld for HelloWorldImpl {
    fn say_hello(&self) {
        println!("Hello, world!");
    }
}
fn main() {
    // 模拟组件的创建和存储
    let hello_world_impl = Arc::new(HelloWorldImpl) as Arc<dyn Any + Send + Sync>;
    let mut components = HashMap::new();
    components.insert("HelloWorldImpl".to_string(), hello_world_impl);

    // 创建模块
    let module = Module { components };

    // 获取组件并调用方法
    let hello_world: &HelloWorldImpl = module.resolve_ref();
    hello_world.say_hello();
}
 

// 在这个简化示例中：
// 1. 定义了一个 `Interface` 特性，它是所有组件特性的父特性。
// 2. 使用 `HashMap` 作为组件存储，键是组件的名称，值是 `Arc<dyn Any + Send + Sync>` 类型的组件。
// 3. `resolve_ref` 方法根据类型名称解析组件，并通过 `downcast_ref` 方法将其转换为特定类型的引用。

// ### 总结

// `resolve_ref` 方法通过使用特性对象和类型反射来实现返回特性的动态引用。它依赖于 Rust 的 `Any` 特性和类型名称来存储和解析组件。当你调用 `resolve_ref` 时，它会查找并返回相应的组件，并将其转换为特性对象的引用。这种设计在保持类型安全的同时，允许在运行时动态解析和使用组件，实现类似于 IoC 容器的功能。

