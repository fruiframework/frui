use std::collections::HashMap;

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Group, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    visit::{self, Visit},
    GenericArgument, Ident, ItemStruct, Type, TypeParam,
};

pub fn derive_builder(tokens: TokenStream1) -> TokenStream1 {
    derive_builder_(syn::parse_macro_input!(tokens)).into()
}

pub fn derive_builder_(item: ItemStruct) -> TokenStream {
    for field in item.fields.iter() {
        assert!(field.ident.is_some(), "tuple struct isn't supported");
        break;
    }

    let name = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    let fields = parse_fields(&item);
    let builder_methods = builder_methods(&fields, &item);

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #builder_methods
        }
    }
}

struct Field {
    ty: TokenStream,
    ident: Ident,
    is_option: bool,
    is_generic: bool,
    generic_ret_ty: TokenStream,
}

fn parse_fields(item: &ItemStruct) -> Vec<Field> {
    item.fields
        .iter()
        .map(|f| {
            let ty = &f.ty;
            let mut field = Field {
                ty: quote!(#ty),
                ident: f.ident.clone().unwrap(),
                is_option: false,
                is_generic: false,
                generic_ret_ty: TokenStream::new(),
            };

            if let Some(ty) = is_option_ty(f) {
                field.ty = quote!(#ty);
                field.is_option = true;

                if let Some((arg_ty, ret_ty)) = is_generic(&ty, &item) {
                    field.ty = arg_ty;
                    field.is_generic = true;
                    field.generic_ret_ty = ret_ty;
                }
            } else {
                if let Some((arg_ty, ret_ty)) = is_generic(&f.ty, &item) {
                    field.ty = arg_ty;
                    field.is_generic = true;
                    field.generic_ret_ty = ret_ty;
                }
            }

            field
        })
        .collect()
}

fn builder_methods(fields: &Vec<Field>, item: &ItemStruct) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let name = &item.ident;

            let ty = &field.ty;
            let ident = &field.ident;
            let ret_ty = &field.generic_ret_ty;

            if field.is_generic {
                let field_idents = field_idents(&item.fields);

                let pass_fields = field_idents.map(|ident| {
                    if ident == &field.ident {
                        if field.is_option {
                            quote!(#ident: Some(#ident))
                        } else {
                            quote!(#ident: #ident)
                        }
                    } else {
                        quote!(#ident: self.#ident)
                    }
                });

                quote! {
                    pub fn #ident(self, #ident: #ty) -> #ret_ty {
                        #name {
                            #(#pass_fields),*
                        }
                    }
                }
            } else {
                let arg = if field.is_option {
                    quote!(Some(#ident))
                } else {
                    quote!(#ident)
                };

                quote! {
                    pub fn #ident(mut self, #ident: #ty) -> Self {
                        self.#ident = #arg;
                        self
                    }
                }
            }
        })
        .collect()
}

fn field_idents<'a>(fields: &'a syn::Fields) -> impl Iterator<Item = &'a Ident> {
    fields.iter().map(|f| f.ident.as_ref().unwrap())
}

#[derive(Clone)]
struct GenericParamIdent {
    /// Position of identifier in `TokenStream`.
    pos: usize,
    /// Type parameter for identifier of this type identifier.
    param: TypeParam,
    /// Position of this type parameter in structure's type parameters.
    param_pos: usize,
}

/// Walk a type in search for generic arguments.
struct TypeVisitor {
    /// Used to count identifiers in [`TokenStream`], so we can later swap them
    /// for `impl Type` by iterating through [`TokenStream`].
    ident_counter: usize,
    /// List of generic identifiers found in a provided type.
    generic_ident: Vec<GenericParamIdent>,
    /// Map of all generic type parameters of a structure and their position
    /// parameters list.
    generic_params: HashMap<Ident, (usize, TypeParam)>,
}

impl<'ast> Visit<'ast> for TypeVisitor {
    fn visit_ident(&mut self, i: &'ast Ident) {
        visit::visit_ident(self, i);

        // Increment for each identifier visited.
        self.ident_counter += 1;
    }

    fn visit_type_path(&mut self, i: &'ast syn::TypePath) {
        if i.path.segments.len() == 1 {
            let seg = i.path.segments.last().unwrap();

            if seg.arguments.is_empty() {
                if let Some((param_pos, t)) = self.generic_params.get(&seg.ident) {
                    //
                    // Generic paramter found, push into a queue.
                    self.generic_ident.push(GenericParamIdent {
                        pos: self.ident_counter + 1,
                        param: t.clone(),
                        param_pos: *param_pos,
                    });
                }
            }
        }

        visit::visit_type_path(self, i);
    }
}

fn is_generic(ty: &Type, item: &ItemStruct) -> Option<(TokenStream, TokenStream)> {
    // HashMap of all generic parameters in a given structure and their position
    // in the parameter list.
    let map: HashMap<Ident, (usize, TypeParam)> = HashMap::from_iter(
        item.generics
            .params
            .iter()
            .enumerate()
            .filter_map(|(n, param)| {
                if let syn::GenericParam::Type(param) = param {
                    Some((param.ident.clone(), (n, param.clone())))
                } else {
                    None
                }
            }),
    );

    //
    // Find generic arguments in a given type (of a field).

    let mut visitor = TypeVisitor {
        ident_counter: 0,
        generic_ident: Vec::with_capacity(map.len() * 2),
        generic_params: map,
    };

    visitor.visit_type(ty);

    if visitor.generic_ident.is_empty() {
        return None;
    }

    //
    // Construct type of an argument of a builder method for this field.
    // E.g.: replace generic identifier `T` with impl Bounds.

    let mut ident_idx = 0;

    let field_ty = ty
        .to_token_stream()
        .into_iter()
        .map(|tt| swap_param_for_impl(tt, &mut ident_idx, &mut visitor.generic_ident.clone()))
        .collect::<TokenStream>();

    //
    // Construct return type of a builder method for this field.
    // E.g.: replace generic identifier `T` with impl Bounds.

    let name = &item.ident;

    let params = item
        .generics
        .type_params()
        .into_iter()
        .enumerate()
        .map(|(i, p)| {
            // Check if this generic parameter appeared in the field type.
            let r = visitor.generic_ident.iter().find(|t| t.param_pos == i);

            if let Some(p) = r {
                let bounds = &p.param.bounds;
                quote!(impl #bounds)
            } else {
                let ident = &p.ident;
                quote!(#ident)
            }
        });

    Some((field_ty, quote!(#name<#(#params),*>)))
}

fn swap_param_for_impl(
    tt: TokenTree,
    ident_idx: &mut usize,
    generic_ident: &mut Vec<GenericParamIdent>,
) -> TokenTree {
    match tt {
        TokenTree::Ident(_) => {
            // Increment for each identifier visited.
            *ident_idx += 1;

            // If this identifier is a generic one, swap it with impl Type.
            if let Some(GenericParamIdent { pos, .. }) = generic_ident.get(0) {
                if ident_idx == pos {
                    let bounds = generic_ident.remove(0).param.bounds;

                    let impl_ty = quote!((impl #bounds));

                    //
                    // Turn `TokenStream` into `TokenTree` for the return type.

                    let tt = impl_ty.into_iter().last().unwrap();

                    match tt {
                        TokenTree::Group(_) => return tt,
                        _ => unreachable!(),
                    }
                }
            }
        }
        // Recurse into a group.
        TokenTree::Group(ref g) => {
            return TokenTree::Group(Group::new(
                g.delimiter(),
                g.stream()
                    .into_iter()
                    .map(|tt| swap_param_for_impl(tt, ident_idx, generic_ident))
                    .collect(),
            ))
        }
        _ => {}
    }

    // Otherwise pass token as is.
    return tt;
}

fn is_option_ty(field: &syn::Field) -> Option<Type> {
    if let Type::Path(p) = &field.ty {
        let type_ident = &p.path.segments.last().unwrap().ident;

        // Check type identifier.
        if type_ident == "Option" {
            let arg = &p.path.segments.last().unwrap().arguments;

            // Extract T from Option<T>.
            if let syn::PathArguments::AngleBracketed(arg) = arg {
                let arg = arg.args.first().unwrap();

                if let GenericArgument::Type(ty) = arg {
                    return Some(ty.clone());
                }
            }
        }
    }

    None
}
