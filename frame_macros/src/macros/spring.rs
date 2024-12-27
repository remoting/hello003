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

pub fn singleton(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

    // 添加一个字段，用于存储方法
    let methods_field: syn::Field = syn::Field::parse_named.parse2(quote! { methods: std::collections::HashMap<String, fn(&#name, String) -> String> }).unwrap();
    
    if let Fields::Named(ref mut fields) = input.fields {
        fields.named.push(methods_field);
    }

    let fields_init = input.fields.iter().map(|f| {
        let name = &f.ident;
        if name.as_ref().unwrap() == "methods" {
            quote! { #name: std::collections::HashMap::new() }
        } else {
            quote! { #name: Default::default() }
        }
    }).collect::<Vec<_>>();

    // 获取结构体名称
    let register_fn_name = syn::Ident::new(&format!("register_singleton_{}", name), name.span());

    let expanded = quote! {
        #input
        impl #name {
            fn new() -> Self {
                #name {
                    #(#fields_init),*
                }
            }
            pub fn get_method(&self, name: &str) -> Option<fn(&#name, String) -> String> {
                self.methods.get(name).cloned()
            }
            pub fn get_instance() -> std::sync::Arc<std::sync::RwLock<Self>> {
                frame::spring::get_instance_by_type::<Self>().unwrap()
            }
        }

        #[ctor::ctor]
        fn #register_fn_name() {
            let instance = std::sync::Arc::new(std::sync::RwLock::new(#name::new()));
            frame::spring::register_instance_by_type(instance.clone());
        }
    };

    TokenStream::from(expanded)
}