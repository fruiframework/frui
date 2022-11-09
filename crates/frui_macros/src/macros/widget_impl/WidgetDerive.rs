use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use syn::{GenericParam, Generics, Ident, ItemStruct, Lifetime, LifetimeDef, TypeParamBound};

use super::unique_type_id;

pub fn impl_widget_derive(input: &ItemStruct, target: &Ident) -> TokenStream2 {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let super::Imports {
        LT,
        Widget,
        WidgetDerive,
        ..
    } = super::imports();

    let Generated {
        Alias,
        AliasWhere,
        AliasParams,
        AssocParams,
    } = generate(target, &input.generics);

    let UniqueTypeId = unique_type_id(target);

    quote! {
        // At the moment of implementing this, TAIT (Type Alias Implement Trait feature)
        // doesn't work inside of the definition of `WidgetDerive` implementation.
        //
        // As a workaround we use a secondary type alias that is outside of `WidgetDerive`
        // definition, which helps the compiler to correctly infer type from `build` method.
        //
        #[doc(hidden)]
        type #Alias #AliasParams #AliasWhere = impl #Widget + #LT;

        impl #impl_generics #WidgetDerive for #target #ty_generics #where_clause {
            type Widget<#LT> = #Alias #AssocParams where Self: #LT;

            type UniqueTypeId = #UniqueTypeId;
        }
    }
}

struct Generated {
    Alias: Ident,
    AliasWhere: TokenStream2,
    AliasParams: Generics,
    AssocParams: TokenStream2,
}

fn generate(name: &Ident, generics: &Generics) -> Generated {
    let mut AliasParams = generics.clone();

    for param in AliasParams.type_params_mut() {
        // Add 'frui for each generic bound.
        //
        // E.g.: <BoundA + 'frui, BoundB + 'static + 'frui>
        param.bounds.push(frui_lt_t());
    }

    AliasParams.params.insert(0, frui_lt_g());

    let AssocParams = {
        let mut generics = generics.clone();
        generics.params.insert(0, frui_lt_g());
        generics.split_for_impl().1.to_token_stream()
    };

    let Alias = format_ident!("FruiInferWidgetFor{}", name);

    Generated {
        Alias,
        AliasWhere: AliasParams.split_for_impl().2.to_token_stream(),
        AliasParams,
        AssocParams,
    }
}

fn frui_lt_t() -> TypeParamBound {
    TypeParamBound::Lifetime(Lifetime::new("'frui", Span::call_site()))
}

fn frui_lt_g() -> GenericParam {
    GenericParam::Lifetime(LifetimeDef::new(Lifetime::new("'frui", Span::call_site())))
}
