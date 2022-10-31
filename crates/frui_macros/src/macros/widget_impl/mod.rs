#![allow(bad_style)]

mod RawWidget;
mod StructuralEq;
mod WidgetDerive;

use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_crate::FoundCrate;
use quote::{format_ident, quote};
use syn::ItemStruct;

use self::{
    RawWidget::impl_raw_widget, StructuralEq::impl_structural_eq, WidgetDerive::impl_widget_derive,
};

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

fn impl_widget(input: &ItemStruct, kind: WidgetKind) -> TokenStream {
    let Target = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let Imports {
        LT,
        Widget,
        RawWidget,
        ..
    } = imports();

    let UniqueTypeId = unique_type_id(Target);

    let RawWidgetImplementation = impl_raw_widget(input, kind);
    let StructuralEqImplementation = impl_structural_eq(input);
    let WidgetDeriveImplementation = impl_widget_derive(kind, input, Target);

    quote! {
        // Combine different WidgetKind implementations into one `RawWidget`.
        #RawWidgetImplementation

        // Implement StructuralEq
        #StructuralEqImplementation

        // Automatically derive `type Self::Widget<'a> = impl Widget`.
        #WidgetDeriveImplementation

        #[doc(hidden)]
        pub enum #UniqueTypeId {}

        impl #impl_generics #Widget for #Target #ty_generics #where_clause {
            fn unique_type(&self) -> ::std::any::TypeId {
                ::std::any::TypeId::of::<#UniqueTypeId>()
            }

            fn as_raw<#LT>(&#LT self) -> &#LT dyn #RawWidget {
                self
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
    RawWidget: TokenStream,
    WidgetDerive: TokenStream,
}

fn imports() -> Imports {
    let exports = exports_path();

    Imports {
        LT: quote! { 'frui },
        Widget: quote! { #exports::Widget },
        RawWidget: quote! { #exports::RawWidget },
        WidgetDerive: quote! { #exports::WidgetDerive },
    }
}
