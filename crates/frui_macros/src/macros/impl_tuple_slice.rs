use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::Parse, Expr, ExprRange};

pub fn impl_tuple_slice(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as Range);

    let o = (input.start..input.end).map(|args_count| {
        let bounds_1 = (0..args_count).map(|i| format_ident!("_{i}"));
        let bounds_2 = (0..args_count).map(|i| format_ident!("_{i}"));
        let bounds_3 = (0..args_count).map(syn::Index::from);

        quote! {
            impl< #(#bounds_1 : Widget),* > WidgetList for ( #(#bounds_2,)* ) {
                fn get(&self) -> Vec<&dyn Widget> {
                    vec![ #( &self.#bounds_3 ),* ]
                }
            }
        }
    });

    (quote! { #(#o)* }).into()
}

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Parse for Range {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let range = input.parse::<ExprRange>()?;
        Ok(Self {
            start: extract_usize(&range.from),
            end: extract_usize(&range.to),
        })
    }
}

fn extract_usize(range: &Option<Box<Expr>>) -> usize {
    match range.as_ref().unwrap().as_ref() {
        Expr::Lit(lit) => match &lit.lit {
            syn::Lit::Int(v) => v.base10_parse().unwrap(),
            _ => panic!(),
        },
        _ => panic!(),
    }
}
