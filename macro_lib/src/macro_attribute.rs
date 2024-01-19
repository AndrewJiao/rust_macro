use proc_macro::TokenStream;
use std::collections::HashMap;

use quote::{quote, ToTokens};
use syn::{FnArg, Ident, ItemFn, LitStr, parse2, Pat, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

// #[get_method(Get, method = "/api/v1")]
impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let result = Punctuated::<(String, String), Token![,]>::parse_terminated_with
            (input
             , |e| {
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

pub  fn parse_get_method(args: Args, item_fn: &mut ItemFn) -> TokenStream {
    let signature = &item_fn.sig;
    let ident = &signature.ident.to_string();
    let origin_block = &item_fn.block;
    let params = &signature.inputs;

    let params = params.into_iter().filter_map(|e| {
        if let FnArg::Typed(e) = e {
            if let Pat::Ident(e) = *(e.clone().pat) {
                let ident = e.ident;
                Some(quote!({
                    println!("^^^^param name =  {}", #ident);
                }))
            } else { None }
        } else { None }
    }).reduce(|first, second| {
        quote!(
            #first
            #second
        )
    }).unwrap_or_else(|| quote!({}));

    let method = format!("{:?}", args.method);
    let path = args.path;

    let new_block = quote! {
    {
        #params
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
pub struct Args {
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
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => { Method::GET }
        };
    }
}

