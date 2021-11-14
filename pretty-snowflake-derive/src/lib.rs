use proc_macro::{self, TokenStream};
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(Label)]
pub fn label_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = syn::parse_macro_input!(input);
    let output = quote! {
        impl pretty_snowflake::Label for #ident {
            type Labeler = pretty_snowflake::MakeLabeling<Self>;
            fn labeler() -> Self::Labeler { pretty_snowflake::MakeLabeling::default() }
        }
    };
    output.into()
}
