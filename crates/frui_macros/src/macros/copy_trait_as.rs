use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::ToTokens;
use syn::{parse::Parse, punctuated::Punctuated, ItemTrait, Token};

pub fn copy_trait_as(body: TokenStream, args: TokenStream) -> TokenStream {
    let mut item: ItemTrait = syn::parse_macro_input!(body);
    let idents: Identifiers = syn::parse_macro_input!(args);

    let mut tokens = TokenStream2::new();

    for ident in idents.0 {
        item.ident = ident;
        item.to_tokens(&mut tokens);
    }

    tokens.into()
}

struct Identifiers(Punctuated<Ident, Token![,]>);

impl Parse for Identifiers {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let v = input.parse_terminated(Ident::parse)?;
        Ok(Self(v))
    }
}
