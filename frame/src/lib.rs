use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemStruct, Fields};

#[proc_macro_derive(spring)]
pub fn spring(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl #name {
            pub fn hello(&self) {
                println!("Hello from {}", stringify!(#name));
            }
        }
    };

    TokenStream::from(expanded)
}

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