// 定义一个过程宏的属性
#[proc_macro_derive(MyDebug)]
pub fn derive_my_debug(input: TokenStream) -> TokenStream {
    // 解析输入的 TokenStream，获取结构体的信息
    let ast = syn::parse(input).unwrap();

    // 生成实现 Debug trait 的代码
    impl_debug_macro(&ast)
}

// 辅助函数，用于生成实现 Debug trait 的代码
fn impl_debug_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("MyStruct")
                    .field("value", &self.value)
                    .finish()
            }
        }
    };
    gen.into()
}

// 使用自定义派生宏
#[derive(MyDebug)]
struct MyStruct {
    value: i32,
}

fn main() {
    let my_struct = MyStruct { value: 42 };
    println!("{:?}", my_struct);
}
