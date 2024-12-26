# my-rust-project

这是一个包含两个Rust项目的工作空间：`frame` 和 `app`。

## 项目结构

```
my-rust-project
├── frame          # 属性宏定义项目
│   ├── src
│   │   └── lib.rs
│   ├── Cargo.toml
│   └── README.md
├── app            # 使用属性宏的应用程序
│   ├── src
│   │   └── main.rs
│   ├── Cargo.toml
│   └── README.md
└── README.md      # 整个项目的文档
```

## 项目说明

- **frame**: 该项目用于定义属性宏，导出一个名为 `my_macro` 的宏，供其他项目使用。
- **app**: 该项目是一个应用程序，使用 `frame` 项目中的 `my_macro` 宏。

## 使用说明

请参阅各个子项目的 `README.md` 文件以获取详细的使用说明和配置方法。