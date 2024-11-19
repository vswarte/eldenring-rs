use syn::{ItemStruct, LitStr};
use quote::quote;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn singleton(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct: ItemStruct = syn::parse_macro_input!(input as ItemStruct);
    let input_struct_ident = input_struct.ident.clone();
    let dlrf_name = syn::parse_macro_input!(args as LitStr).value();

    TokenStream::from(quote! {
        #input_struct

        impl ::dlrf::DLRFSingleton for #input_struct_ident {
            const DLRF_NAME: &'static str = #dlrf_name;
        }
    })
}
