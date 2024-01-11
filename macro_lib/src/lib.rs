use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

mod process_macro_attr;

//<editor-fold desc="派生宏模拟">
///
/// 这是一个派生宏demo，核心要点是他们必须在单独的包里定义
/// 只能在lib.rs里面定义宏解析函数
/// 要派生的属性和类尽可能用全路径名称。避免派生后找不到该属性和函数
///
#[proc_macro_derive(SuperTrait)]
pub fn generate_trait(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    do_generate_trait(input)
}

fn do_generate_trait(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    // let data = input.data;
    // let fields = match &input.data {
    //     Data::Struct(data) => { Some(&data.fields) }
    //     Data::Enum(_) => { None }
    //     Data::Union(data) => { Some(&data.fields) }
    // };


    let stream = quote!(
            impl SuperTrait for  #name{
                fn do_some_echo() {
                    println!("struct name is value = {:?}", stringify!(#name))
                }

                fn do_some_obj_echo(&self) {
                    println!("obj echo {}", stringify!(#name));
                }
            }
        );
    TokenStream::from(stream)
}
//</editor-fold>


//<editor-fold desc="属性宏demo">
#[cfg(test)]
mod test_attribute {

    //get attr
    #[test]
    pub fn derive_attribute() {}

    #[]
    fn test_get() {}
}

///
///本节介绍属性宏
/// 属性宏类似java的注解，能接受item作为参数
///
/// 本demo模拟定义一个GET方法的属性宏，并提取其中的uri
///
#[proc_macro_attribute]
pub fn get_method(meta: TokenStream, token: TokenStream) -> TokenStream {
    let stream = parse_macro_input!(meta as );
}

///
/// 首先需要定义一个结构体，用于封装解析的结果
///
struct Args {
    path: String
}
impl Parse for Args {

    fn parse(input: ParseStream) -> syn::Result<Self> {
        let result = Punctuated::<String, Token![,]>::parse_terminated(input)?;
    }

}

//</editor-fold>
