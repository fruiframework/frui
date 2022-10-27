use proc_macro2::{Literal, TokenStream};
use quote::{quote, ToTokens};
use syn::ItemStruct;

use super::exports_path;

pub fn impl_structural_eq(item: &ItemStruct) -> TokenStream {
    let Imports {
        StructuralEqImpl, ..
    } = imports();

    let (eq_enabled, eq_impl) = eq_impl(item.clone());

    let Target = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    quote! {
        unsafe impl #impl_generics #StructuralEqImpl for #Target #ty_generics #where_clause {
            const EQ_ENABLED: bool = #eq_enabled;

            fn eq(&self, other: &Self) -> bool {
                #eq_impl
            }
        }
    }
}

fn eq_impl(input: ItemStruct) -> (bool, TokenStream) {
    let Imports { StructuralEq, .. } = imports();

    // Constant-evaluated (optimized) expression indicating if all fields are cheap to compare.
    // That means that e.g. no field contains another widget, which would cause recursive
    // equality tests of the widget subtree to be performed.
    let fields_cheap_to_eq = input.fields.iter().map(|t| {
        let ty = &t.ty;
        quote!(<#ty as #StructuralEq>::EQ_ENABLED &&)
    });

    let fields_eq = input.fields.iter().enumerate().map(|(n, t)| {
        let field_ident = t.ident.clone().map_or(
            // Unnamed struct field.
            Literal::usize_unsuffixed(n).to_token_stream(),
            // Named struct field.
            |v| v.to_token_stream(),
        );
        quote!(#StructuralEq::eq(&self.#field_ident, &other.#field_ident) &&)
    });

    let eq_impl = quote! (#(#fields_cheap_to_eq)*  #(#fields_eq)* true);

    let cheap_to_cmp = input.fields.len() == 0;

    (cheap_to_cmp, eq_impl)
}

struct Imports {
    StructuralEq: TokenStream,
    StructuralEqImpl: TokenStream,
}

fn imports() -> Imports {
    let exports = exports_path();

    Imports {
        StructuralEq: quote!(#exports::StructuralEq),
        StructuralEqImpl: quote! { #exports::StructuralEqImpl },
    }
}
