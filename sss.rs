
#[proc_macro_attribute]
pub fn log_methods(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let name = &input.self_ty;

    // 获取要检查的标记字符
    let marker = attr.to_string();

    // 获取方法名和属性
    let methods: Vec<String> = input.items.iter().filter_map(|item| {
        if let ImplItem::Method(method) = item {
            // 检查方法是否具有特定的属性标记
            let has_marker = method.attrs.iter().any(|attr| {
                attr.path.is_ident(&marker)
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
    let log_code = quote! {
        {
            use std::fs::OpenOptions;
            use std::io::Write;
            let mut file = OpenOptions::new().append(true).create(true).open("struct_info.log").unwrap();
            writeln!(file, "Methods with marker '{}': {}", #marker, #method_names).unwrap();
        }
    };

    // 生成新的实现块，包含记录方法名的代码
    let expanded = quote! {
        #input

        impl #name {
            pub fn log_methods() {
                #log_code
            }
        }
    };

    TokenStream::from(expanded)
}

#[log_methods(command)]
impl MyStruct1 {
    #[command]
    pub fn method1(&self) {}

    pub fn method2(&self) {}
}