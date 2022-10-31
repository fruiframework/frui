use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Visibility;

pub fn sealed(args: TokenStream, body: TokenStream) -> TokenStream {
    let vis: Visibility = syn::parse_macro_input!(args);
    let body: TokenStream2 = body.into();

    quote! {
        pub(#vis) use sealed::*;

        mod sealed {
            use super::*;

            #body
        }
    }
    .into()
}
