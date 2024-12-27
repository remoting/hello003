use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{parse_macro_input, DeriveInput, Fields, ItemFn, ItemStruct, ItemImpl, ImplItem}; 
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use lazy_static::lazy_static;
use log::info;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;


// 这个宏只是打标记而已
pub fn command(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    // 直接返回输入，不生成任何额外代码
    TokenStream::from(quote::quote! {
        #input
    })
}
// 这个宏用在结构体实现impl块上，用来记录具有特定属性的方法名
pub fn controller(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let name = &input.self_ty;

    // 获取要检查的标记字符
    let marker = _attr.to_string();

     // 获取方法名和属性
     let methods: Vec<String> = input.items.iter().filter_map(|item| {
        if let ImplItem::Fn(method) = item {
            // 检查方法是否具有特定的属性标记
            let has_marker = method.attrs.iter().any(|attr| {
                attr.path().is_ident(&marker)
            });

            if has_marker {
                Some(method.sig.ident.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }).collect();

    // 生成记录方法名的代码
    let method_names = methods.join(", ");
    let mut file = OpenOptions::new().append(true).create(true).open("./logs/struct_info.log").unwrap();
    writeln!(file, "Methods with marker '{}': {}", marker, method_names).unwrap();
    // 生成新的实现块，包含记录方法名的代码
    let expanded = quote! {
        #input
    };

    TokenStream::from(expanded)
}
