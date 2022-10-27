use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use syn::{GenericParam, Generics, Ident, ItemStruct, Lifetime, LifetimeDef, TypeParamBound};

use super::{unique_type_id, WidgetKind};

pub fn impl_widget_derive(kind: WidgetKind, input: &ItemStruct, target: &Ident) -> TokenStream2 {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let super::Imports {
        LT,
        Widget,
        WidgetDerive,
        ..
    } = super::imports();

    let Generated {
        Alias,
        AssocParams,
        AliasParam: AliasGenerics,
        AliasWhere,
    } = generate(target, &input.generics);

    let UniqueTypeId = unique_type_id(target);

    match kind {
        WidgetKind::Leaf => quote! {
            // Leaf widget doesn't have any children, that's why we don't infer their type.
            //
            impl #impl_generics #WidgetDerive for #target #ty_generics #where_clause {
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
            type #Alias #AliasGenerics #AliasWhere = impl #Widget + #LT;

            impl #impl_generics #WidgetDerive for #target #ty_generics #where_clause {
                type Widget<#LT> = #Alias #AssocParams where Self: #LT;

                type UniqueTypeId = #UniqueTypeId;
            }
        },
    }
}

struct Generated {
    /// Identifier of the type alias which contains TAIT for widget type
    /// inference.
    Alias: Ident,
    AliasParam: Generics,
    AliasWhere: TokenStream2,
    /// Associated type defining type of a given widget child.
    AssocParams: TokenStream2,
}

fn generate(name: &Ident, generics: &Generics) -> Generated {
    // Those parameters are the same as in the definition, except that they
    // contain an additional 'frui lifetime.
    //
    // Used inside of `type Alias<*here*> where *here* = impl ...;`
    let mut AliasGenerics = generics.clone();

    for param in AliasGenerics.type_params_mut() {
        // Add 'frui for each lifetime bound.
        //
        // E.g.: <BoundA + 'frui, BoundB + 'static + 'frui>
        param.bounds.push(frui_lt_t());
    }

    // Insert 'frui before above bounds: <'frui, ...>
    AliasGenerics.params.insert(0, frui_lt_g());

    // This was originally used as input type paramteres inside of associated
    // type, used to pass parameters to tyoe alias (TAIT).
    //
    // Used in: `type ChildWidget<'frui>: Alias<here> ...;`
    let mut AssocParams = generics.clone();
    AssocParams.params.insert(0, frui_lt_g());
    let AssocParams = AssocParams.split_for_impl().1.to_token_stream();

    let Alias = format_ident!("FruiInferWidgetFor{}", name);

    Generated {
        Alias,
        AssocParams,
        AliasWhere: AliasGenerics.split_for_impl().2.to_token_stream(),
        AliasParam: AliasGenerics,
    }
}

fn frui_lt_t() -> TypeParamBound {
    TypeParamBound::Lifetime(Lifetime::new("'frui", Span::call_site()))
}

fn frui_lt_g() -> GenericParam {
    GenericParam::Lifetime(LifetimeDef::new(Lifetime::new("'frui", Span::call_site())))
}
