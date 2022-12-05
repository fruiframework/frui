use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

use super::{exports_path, WidgetKind};

pub fn impl_raw_widget(item: &ItemStruct, widget_kind: WidgetKind) -> TokenStream {
    let WidgetKindOS = kind_to_os(widget_kind);

    #[rustfmt::skip]
    let Imports {
        Vec, TypeId,
        RawWidget, WidgetPtr,
        RawBuildCx, LayoutCxOS, PaintCxOS, Canvas, 
        Size, Offset, Constraints, 
    } = imports_impl_widget_os();

    let Target = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    quote! {
        impl #impl_generics #RawWidget for #Target #ty_generics #where_clause {
            fn build<'w>(&'w self, cx: &'w #RawBuildCx) -> #Vec<#WidgetPtr<'w>> {
                <Self as #WidgetKindOS>::build(self, cx)
            }

            fn layout(&self, cx: #LayoutCxOS, constraints: #Constraints) -> #Size {
                <Self as #WidgetKindOS>::layout(self, cx, constraints)
            }

            fn paint(&self, cx: #PaintCxOS, canvas: &mut #Canvas, offset: &#Offset) {
                <Self as #WidgetKindOS>::paint(self, cx, canvas, offset)
            }

            fn inherited_key(&self) -> Option<#TypeId> {
                <Self as #WidgetKindOS>::inherited_key(self)
            }
        }
    }
}

fn kind_to_os(widget_kind: WidgetKind) -> TokenStream {
    let exports = exports_path();

    match widget_kind {
        WidgetKind::View => quote!(#exports::ViewWidgetOS),
        WidgetKind::Inherited => quote!(#exports::InheritedWidgetOS),
        WidgetKind::Render => quote!(#exports::RenderWidgetOS),
    }
}

struct Imports {
    // Standard
    Vec: TokenStream,
    TypeId: TokenStream,
    // Traits
    RawWidget: TokenStream,
    WidgetPtr: TokenStream,
    // Contextes
    RawBuildCx: TokenStream,
    LayoutCxOS: TokenStream,
    Canvas: TokenStream,
    PaintCxOS: TokenStream,
    // Types
    Size: TokenStream,
    Offset: TokenStream,
    Constraints: TokenStream,
}

fn imports_impl_widget_os() -> Imports {
    let exports = exports_path();

    Imports {
        Vec: quote!(::std::vec::Vec),
        TypeId: quote!(::std::any::TypeId),
        RawWidget: quote!(#exports::RawWidget),
        WidgetPtr: quote!(#exports::WidgetPtr),
        RawBuildCx: quote!(#exports::RawBuildCx),
        LayoutCxOS: quote!(#exports::LayoutCxOS),
        Canvas: quote!(#exports::Canvas),
        PaintCxOS: quote!(#exports::PaintCxOS),
        Size: quote!(#exports::Size),
        Offset: quote!(#exports::Offset),
        Constraints: quote!(#exports::Constraints),
    }
}
