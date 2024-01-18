use proc_macro::TokenStream;
use std::cmp::max_by_key;
use std::collections::HashMap;
use std::fmt::Debug;

use quote::{quote, ToTokens};
use syn::{Block, DeriveInput, Ident, ItemFn, LitStr, parse, parse2, parse_macro_input, Token};
use syn::parse::{Parse, ParseStream, Peek};
use syn::punctuated::Punctuated;

use crate::Method::{GET, POST};

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
    let signature = &item_fn.sig;
    let ident = &signature.ident.to_string();
    let origin_block = &item_fn.block;

    let method = format!("{:?}",args.method);;
    let path = args.path;


    let new_block = quote! {
    {
        println!("before fn for name {}", #ident);
        #origin_block
        println!("after fn for name {}, and method = {} and path = {}", #ident, #method, #path)
    }
    };
    item_fn.block = Box::new(parse2(new_block).unwrap());
    item_fn.into_token_stream().into()
}


///
/// 首先需要定义一个结构体，用于封装解析的结果
///
struct Args {
    path: String,
    method: Method,
}

impl Args {
    pub fn new(mut map: HashMap<String, String>) -> Self {
        let path = map.remove("path").unwrap();
        let method = map.remove("method").unwrap().into();
        Self { path, method }
    }
}


#[derive(Debug)]
enum Method {
    GET,
    POST,
}

impl From<String> for Method {
    fn from(value: String) -> Self {
        return match value.as_ref() {
            "GET" => GET,
            "POST" => POST,
            _ => { GET }
        };
    }
}


// #[get_method(Get, method = "/api/v1")]
impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        eprintln!("star {}", input.clone());
        let result = Punctuated::<(String, String), Token![,]>::parse_terminated_with
            (input
             , |e| {
                eprintln!("e_stream = {}", e);
                let key: Ident = e.parse()?;
                e.parse::<Token![=]>()?;
                let str: LitStr = e.parse()?;
                Ok(
                    (
                        key.to_string(),
                        str.to_token_stream().to_string(),
                    )
                )
            })?.into_iter().collect::<HashMap<String, String>>();

        Ok(Args::new(result))
    }
}

//</editor-fold>
