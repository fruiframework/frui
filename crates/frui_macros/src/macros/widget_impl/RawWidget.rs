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
        Context, PaintContext, AnyRenderContext,
        Size, Offset, Constraints, 
    } = imports_impl_widget_os();

    let Target = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    quote! {
        impl #impl_generics #RawWidget for #Target #ty_generics #where_clause {
            fn build<'w>(&'w self, ctx: &'w #Context) -> #Vec<#WidgetPtr<'w>> {
                <Self as #WidgetKindOS>::build(self, ctx)
            }

            fn layout<'w>(&self, ctx: &'w #AnyRenderContext, constraints: #Constraints) -> #Size {
                <Self as #WidgetKindOS>::layout(self, ctx, constraints)
            }

            fn paint<'w>(
                &'w self,
                ctx: &'w #AnyRenderContext,
                canvas: &mut #PaintContext,
                offset: &#Offset
            ) {
                <Self as #WidgetKindOS>::paint(self, ctx, canvas, offset)
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
    Context: TokenStream,
    PaintContext: TokenStream,
    AnyRenderContext: TokenStream,
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
        Context: quote!(#exports::Context),
        PaintContext: quote!(#exports::PaintContext),
        AnyRenderContext: quote!(#exports::AnyRenderContext),
        Size: quote!(#exports::Size),
        Offset: quote!(#exports::Offset),
        Constraints: quote!(#exports::Constraints),
    }
}
