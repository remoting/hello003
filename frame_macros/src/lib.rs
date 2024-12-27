use proc_macro::TokenStream;
mod macros;
// 这个宏只是打标记而已
#[proc_macro_attribute]
pub fn command(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    macros::tauri::command(_attr, _item)
}
// 这个宏用在结构体实现impl块上，用来记录具有特定属性的方法名
#[proc_macro_attribute]
pub fn controller(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    macros::tauri::controller(_attr, _item)
}
// 这个宏是用来做测试用的
#[proc_macro_derive(hello)]
pub fn hello(input: TokenStream) -> TokenStream {
    macros::hello::hello(input)
}

// singleton 实现 类似Spring 的单例  
#[proc_macro_attribute]
pub fn singleton(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    macros::spring::singleton(_attr, _item)
}

// #[proc_macro_attribute]
// pub fn register_command(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(item as ItemFn);
//     let name = &input.sig.ident;
//     let name_str = name.to_string();

//     let expanded = quote! {
//         #input

//         #[ctor::ctor]
//         fn register_command() {
//             frame_support::register_function(#name_str, #name as fn());
//         }
//     };

//     TokenStream::from(expanded)
// }