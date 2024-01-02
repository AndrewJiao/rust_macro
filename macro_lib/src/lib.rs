use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}


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

