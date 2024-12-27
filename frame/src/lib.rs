use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, ItemFn, ItemStruct, ItemImpl, ImplItem}; 
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use lazy_static::lazy_static;
use log::info;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;

// 这个宏只是打标记而已
#[proc_macro_attribute]
pub fn command(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    // 直接返回输入，不生成任何额外代码
    TokenStream::from(quote::quote! {
        #input
    })
}
// 这个宏用在结构体实现impl块上，用来记录具有特定属性的方法名
#[proc_macro_attribute]
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

#[proc_macro_derive(hello)]
pub fn hello(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
 
    let expanded = quote! {
        impl #name {
            pub fn hello_method(&self) -> String {
                format!("hello method called with field: {}", self.field)
            }
        }
    };

    TokenStream::from(expanded)
}

// 保留 singleton 宏的原始实现
#[proc_macro_attribute]
pub fn singleton(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;
    let fields = if let Fields::Named(fields) = &input.fields {
        fields.named.iter().map(|f| {
            let name = &f.ident;
            quote! { #name: Default::default() }
        }).collect::<Vec<_>>()
    } else {
        vec![]
    };
    let register_fn_name = syn::Ident::new(&format!("register_singleton_{}", name), name.span());

    let expanded = quote! {
        #input

        impl #name {
            fn new() -> Self {
                #name {
                    #(#fields),*
                }
            }

            pub fn get_instance() -> std::sync::Arc<std::sync::RwLock<Self>> {
                frame_support::get_instance_by_type::<Self>().unwrap()
            }
        }

        #[ctor::ctor]
        fn #register_fn_name() {
            let instance = std::sync::Arc::new(std::sync::RwLock::new(#name::new()));
            frame_support::register_instance_by_type(instance.clone());
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn register_command(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let name_str = name.to_string();

    let expanded = quote! {
        #input

        #[ctor::ctor]
        fn register_command() {
            frame_support::register_function(#name_str, #name as fn());
        }
    };

    TokenStream::from(expanded)
}