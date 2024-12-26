# frame 项目文档

该项目定义了一个属性宏 `my_macro`，可以在其他 Rust 项目中使用。

## 安装

在你的 `Cargo.toml` 文件中添加以下依赖：

```toml
[dependencies]
frame = { path = "../frame" }
```

## 使用

在你的 Rust 代码中导入宏并应用于结构体：

```rust
use frame::my_macro;

#[my_macro]
struct MyStruct {
    // 字段定义
}
```

## 示例

请查看 `app` 项目中的示例代码，了解如何使用 `my_macro`。