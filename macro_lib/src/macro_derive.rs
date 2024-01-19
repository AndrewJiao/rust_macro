use proc_macro::TokenStream;

use quote::quote;
use syn::DeriveInput;

pub fn do_generate_trait(input: DeriveInput) -> TokenStream {
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
