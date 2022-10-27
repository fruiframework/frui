#![allow(bad_style)]

mod StructuralEq;
mod WidgetDerive;

use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_crate::FoundCrate;
use quote::{format_ident, quote, ToTokens};
use syn::ItemStruct;

use self::{StructuralEq::impl_structural_eq, WidgetDerive::impl_widget_derive};

//
// Exports

pub fn Leaf(structure: &ItemStruct) -> TokenStream {
    impl_widget(structure, WidgetKind::Leaf)
}

pub fn SingleChild(structure: &ItemStruct) -> TokenStream {
    impl_widget(structure, WidgetKind::SingleChild)
}

pub fn MultiChild(structure: &ItemStruct) -> TokenStream {
    impl_widget(structure, WidgetKind::MultiChild)
}

pub fn View(structure: &ItemStruct) -> TokenStream {
    impl_widget(structure, WidgetKind::View)
}

pub fn Inherited(structure: &ItemStruct) -> TokenStream {
    impl_widget(structure, WidgetKind::Inherited)
}

//
// Impl

#[derive(Debug, Clone, Copy)]
pub enum WidgetKind {
    Leaf,
    SingleChild,
    MultiChild,
    View,
    Inherited,
}

impl ToTokens for WidgetKind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let variant = match self {
            WidgetKind::Leaf => quote!(Leaf),
            WidgetKind::SingleChild => quote!(SingleChild),
            WidgetKind::MultiChild => quote!(MultiChild),
            WidgetKind::View => quote!(View),
            WidgetKind::Inherited => quote!(Inherited),
        };
        tokens.extend(Some(variant))
    }
}

fn impl_widget(input: &ItemStruct, kind: WidgetKind) -> TokenStream {
    let Target = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let Imports {
        LT,
        Widget,
        WidgetKind,
        ..
    } = imports();

    let UniqueTypeId = unique_type_id(Target);
    let WidgetKindVariant = kind;

    let StructuralEqImplementation = impl_structural_eq(input);
    let WidgetDeriveImplementation = impl_widget_derive(kind, input, Target);

    quote! {
        // Implement StructuralEq.
        #StructuralEqImplementation

        // Automatically derive `type Self::Widget<'a> = impl Widget`.
        #WidgetDeriveImplementation

        #[doc(hidden)]
        pub enum #UniqueTypeId {}

        impl #impl_generics #Widget for #Target #ty_generics #where_clause {
            fn unique_type(&self) -> ::std::any::TypeId {
                ::std::any::TypeId::of::<#UniqueTypeId>()
            }

            fn kind<#LT>(&#LT self) -> #WidgetKind {
                #WidgetKind::#WidgetKindVariant(self)
            }
        }
    }
}

pub fn exports_path() -> TokenStream {
    let frui = proc_macro_crate::crate_name("frui");
    let frui_core = proc_macro_crate::crate_name("frui_core");
    let frui_widgets = proc_macro_crate::crate_name("frui_widgets");

    let frui = match (frui, frui_core, frui_widgets) {
        (Ok(f), _, _) => into_ident(f),
        (_, Ok(f), _) => into_ident(f),
        (_, _, Ok(f)) => into_ident(f),
        (Err(_), Err(_), Err(_)) => panic!("couldn't locate frui crate path"),
    };

    fn into_ident(crate_: FoundCrate) -> TokenStream {
        match crate_ {
            FoundCrate::Itself => quote!(crate),
            FoundCrate::Name(name) => {
                let ident = Ident::new(&name, Span::call_site());
                quote!( ::#ident )
            }
        }
    }

    quote!(#frui::macro_exports)
}

fn unique_type_id(name: &Ident) -> Ident {
    format_ident!("FruiUniqueTypeIdFor{}", name)
}

struct Imports {
    LT: TokenStream,
    Widget: TokenStream,
    WidgetKind: TokenStream,
    WidgetDerive: TokenStream,
}

fn imports() -> Imports {
    let exports = exports_path();

    Imports {
        LT: quote! { 'frui },
        Widget: quote! { #exports::Widget },
        WidgetKind: quote! { #exports::WidgetKind },
        WidgetDerive: quote! { #exports::WidgetDerive },
    }
}
