use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, ItemFn, ItemStruct, ItemImpl, ImplItem, Item}; 
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use lazy_static::lazy_static;
use log::info;
use std::fs::File;
use std::io::Write;

#[proc_macro_attribute]
pub fn command(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let name_str = name.to_string();
    let register_fn_name = syn::Ident::new(&format!("{}_{}", env!("CARGO_PKG_NAME"), name), name.span());

    let expanded = quote! {
        #input

        #[ctor::ctor]
        fn #register_fn_name() {
           frame_support::register_function(#name_str, #name as fn());
        }
    };

    TokenStream::from(expanded)
}


#[proc_macro_derive(spring)]
pub fn spring(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // 解析 DeriveInput 中的实现块
    let mut methods = Vec::new();
    for attr in &input.attrs {
        if let Ok(Item::Impl(impl_block)) = syn::parse2::<Item>(attr.tokens.clone()) {
            if *impl_block.self_ty == syn::parse_quote!(#name) {
                for item in impl_block.items {
                    if let ImplItem::Method(method) = item {
                        let method_name = &method.sig.ident;
                        // 记录方法名称
                        methods.push(method_name.to_string());
                    }
                }
            }
        }
    }

    // 输出方法名称到文件
    let mut file = std::fs::File::create("macro_debug.log").unwrap();
    for method in &methods {
        writeln!(file, "Method: {}", method).unwrap();
    }

    let expanded = quote! {
        impl #name {
            pub fn spring_method(&self) -> String {
                format!("Spring method called with field: {}", self.field)
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