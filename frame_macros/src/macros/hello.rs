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