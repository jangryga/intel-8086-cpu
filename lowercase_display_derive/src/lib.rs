extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput};

#[proc_macro_derive(LowercaseDisplay)]
pub fn derive_display_lowercase(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let data = &input.data;

    let arms = match data {
        syn::Data::Enum(e) => e
            .variants
            .iter()
            .map(|v| {
                let ident = &v.ident;
                let ident_str = format!("{}", ident).to_lowercase();
                quote! {
                    #name::#ident => write!(f, #ident_str),
                }
            })
            .collect::<Vec<_>>(),
        _ => panic!("LowercaseDisplay is only implemented for enums"),
    };

    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match *self {
                    #(#arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}