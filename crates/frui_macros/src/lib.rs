use proc_macro::TokenStream;

mod macros;

#[proc_macro]
pub fn impl_widget_list(tokens: TokenStream) -> TokenStream {
    macros::impl_tuple_slice::impl_tuple_slice(tokens)
}

//
// Widget Implementations
//

#[proc_macro_derive(LeafWidget)]
pub fn leaf_widget(tokens: TokenStream) -> TokenStream {
    macros::widget::Leaf(&syn::parse_macro_input!(tokens as syn::ItemStruct)).into()
}

#[proc_macro_derive(SingleChildWidget)]
pub fn single_child_widget(tokens: TokenStream) -> TokenStream {
    macros::widget::SingleChild(&syn::parse_macro_input!(tokens as syn::ItemStruct)).into()
}

#[proc_macro_derive(MultiChildWidget)]
pub fn multi_child_widget(tokens: TokenStream) -> TokenStream {
    macros::widget::MultiChild(&syn::parse_macro_input!(tokens as syn::ItemStruct)).into()
}

#[proc_macro_derive(ViewWidget)]
pub fn view_widget(tokens: TokenStream) -> TokenStream {
    macros::widget::View(&syn::parse_macro_input!(tokens as syn::ItemStruct)).into()
}

#[proc_macro_derive(InheritedWidget)]
pub fn inherited_widget(tokens: TokenStream) -> TokenStream {
    macros::widget::Inherited(&syn::parse_macro_input!(tokens as syn::ItemStruct)).into()
}
