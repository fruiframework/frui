#![allow(bad_style)]

use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{GenericParam, Generics, ItemStruct, Lifetime, LifetimeDef, TypeParamBound};

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

fn impl_widget(input: &ItemStruct, kind: WidgetKind) -> TokenStream {
    let name = &input.ident;
    let (LT, Widget, WidgetKind, _, StructuralEqImpl, _) = imports();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let WidgetKindVariant = kind.into_token();
    let (eq_enabled, eq_impl) = eq_impl(input.clone());
    let WidgetDeriveImpl = widget_derive_impl(kind, input, name);
    let (UniqueTypeId, _, _, _) = widget_derive_helpers(name, &input.generics);

    quote! {
        #[doc(hidden)]
        pub enum #UniqueTypeId {}

        impl #impl_generics #Widget for #name #ty_generics #where_clause {
            fn unique_type(&self) -> ::std::any::TypeId {
                ::std::any::TypeId::of::<#UniqueTypeId>()
            }

            fn kind<#LT>(&#LT self) -> #WidgetKind {
                #WidgetKind::#WidgetKindVariant(self)
            }
        }

        // Automatically derive `type Self::Widget<'a> = impl Widget`.
        #WidgetDeriveImpl

        unsafe impl #impl_generics #StructuralEqImpl for #name #ty_generics #where_clause {
            const EQ_ENABLED: bool = #eq_enabled;

            fn eq(&self, other: &Self) -> bool {
                #eq_impl
            }
        }
    }
}

fn widget_derive_impl(kind: WidgetKind, input: &ItemStruct, name: &Ident) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (LT, Widget, _, _, _, WidgetDerive) = imports();

    let (UniqueTypeId, WidgetAlias, AliasImplBounds, AliasTypeBounds) =
        widget_derive_helpers(name, &input.generics);
    let AliasTypeBounds = AliasTypeBounds.split_for_impl().1;

    match kind {
        WidgetKind::Leaf => quote! {
            // Leaf widget doesn't have any children, that's why we don't infer their type.
            //
            impl #impl_generics #WidgetDerive for #name #ty_generics #where_clause {
                type Widget<#LT> = () where Self: #LT;

                type UniqueTypeId = #UniqueTypeId;
            }
        },
        _ => quote! {
            // At the moment of implementing this, TAIT (Type Alias Implement Trait feature)
            // doesn't work inside of the definition of `WidgetDerive` implementation.
            //
            // As a workaround we use a secondary type alias that is outside of `WidgetDerive`
            // definition, which helps the compiler to correctly infer type from `build` method.
            //
            // This however requires that used type parameters must be explicitly defined
            // in the type alias. Those are `AliasImplBounds` and `AliasTypeBounds` here.
            //
            #[doc(hidden)]
            type #WidgetAlias #AliasImplBounds = impl #Widget + #LT;

            impl #impl_generics #WidgetDerive for #name #ty_generics #where_clause {
                type Widget<#LT> = #WidgetAlias #AliasTypeBounds where Self: #LT;

                type UniqueTypeId = #UniqueTypeId;
            }
        },
    }
}

fn widget_derive_helpers(name: &Ident, gparam: &Generics) -> (Ident, Ident, Generics, Generics) {
    // "Asdavbmwqeryuiopvzxc" is an identifier no one will dare to create ;)
    let UniqueTypeId = format_ident!("Asdavbmwqeryuiopvzxc{}{}", "UniqueTypeId", name);
    let alias_ident = format_ident!("Asdavbmwqeryuiopvzxc{}{}", "WidgetAlias", name);

    let mut impl_param = gparam.clone();

    for param in impl_param.type_params_mut() {
        // We need to add appropriate lifetime requirements for each generic bound.
        param.bounds.push(TypeParamBound::Lifetime(Lifetime::new(
            "'frui",
            Span::call_site(),
        )));
    }

    impl_param
        .params
        .push(GenericParam::Lifetime(LifetimeDef::new(Lifetime::new(
            "'frui",
            Span::call_site(),
        ))));

    //
    //

    let mut type_param = gparam.clone();

    type_param
        .params
        .push(GenericParam::Lifetime(LifetimeDef::new(Lifetime::new(
            "'frui",
            Span::call_site(),
        ))));

    (UniqueTypeId, alias_ident, impl_param, type_param)
}

fn eq_impl(input: ItemStruct) -> (bool, TokenStream) {
    let (_, _, _, StructuralEq, _, _) = imports();

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

fn imports() -> (
    TokenStream,
    TokenStream,
    TokenStream,
    TokenStream,
    TokenStream,
    TokenStream,
) {
    let prelude = quote! { ::frui::prelude };

    let Widget = quote! { #prelude::Widget };
    let WidgetKind = quote! { #prelude::WidgetKind };

    let LT = quote! { 'frui };

    let macro_path = quote! { ::frui::macro_exports };

    let StructuralEq = quote! { #macro_path::StructuralEq };
    let StructuralEqImpl = quote! { #macro_path::StructuralEqImpl };
    let MissingWidgetDerive = quote! { #macro_path::WidgetDerive };

    (
        LT,
        Widget,
        WidgetKind,
        StructuralEq,
        StructuralEqImpl,
        MissingWidgetDerive,
    )
}

#[derive(Debug, Clone, Copy)]
enum WidgetKind {
    Leaf,
    SingleChild,
    MultiChild,
    View,
    Inherited,
}

impl WidgetKind {
    fn into_token(self) -> TokenStream {
        match self {
            WidgetKind::Leaf => quote! { Leaf },
            WidgetKind::SingleChild => quote! { SingleChild },
            WidgetKind::MultiChild => quote! { MultiChild },
            WidgetKind::View => quote! { View },
            WidgetKind::Inherited => quote! { Inherited },
        }
    }
}
