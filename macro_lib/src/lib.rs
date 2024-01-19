use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, ItemFn, parse_macro_input};

use crate::macro_attribute::Args;

mod macro_attribute;
mod macro_derive;

//<editor-fold desc="派生宏模拟">
///
/// 这是一个派生宏demo，核心要点是他们必须在单独的包里定义
/// 只能在lib.rs里面定义宏解析函数
/// 要派生的属性和类尽可能用全路径名称。避免派生后找不到该属性和函数
///
#[proc_macro_derive(SuperTrait)]
pub fn generate_trait(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    macro_derive::do_generate_trait(input)
}

//</editor-fold>


///
///
/// # Arguments
///
/// * `meta`:
/// * `token`:
///
/// returns: TokenStream
///
/// # Examples
///
/// ```
///
/// ```
//<editor-fold desc="属性宏demo">
///
///本节介绍属性宏
/// 属性宏类似java的注解，能接受item作为参数
///
/// 本demo模拟定义一个GET方法的属性宏，并提取其中的uri
///
#[proc_macro_attribute]
pub fn get_method(meta: TokenStream, token: TokenStream) -> TokenStream {
    let args = parse_macro_input!(meta as Args);
    let mut item_fn = parse_macro_input!(token as ItemFn);
    macro_attribute::parse_get_method(args, &mut item_fn)
}



//</editor-fold>


///
/// 函数宏类似申明宏，通过!调用
///
#[proc_macro]
pub fn a_proc_macro(_input: TokenStream) -> TokenStream {
    TokenStream::from(quote!(
            fn anwser()->i32{
                5
            }
))
}