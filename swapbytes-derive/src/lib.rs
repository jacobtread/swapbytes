use darling::FromAttributes;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DataStruct, DeriveInput};

#[derive(Debug, FromAttributes)]
#[darling(attributes(sb), forward_attrs(allow, doc, cfg))]
struct FieldAttr {
    #[darling(default)]
    skip: bool,
}

#[proc_macro_derive(SwapBytes, attributes(sb))]
pub fn derive_swap_bytes(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let data: &DataStruct = match &input.data {
        syn::Data::Struct(value) => value,
        _ => panic!("SwapBytesMut only supported on structs"),
    };

    let ident = &input.ident;
    let generics = &input.generics;
    let where_clause = generics.where_clause.as_ref();

    let swap_bytes_impls = data
        .fields
        .iter()
        .filter(|value| {
            let attr = FieldAttr::from_attributes(&value.attrs).expect("Failed to parse attrs");
            !attr.skip
        })
        .map(|value| {
            let ident = value.ident.as_ref().expect("Struct field missing ident");
            quote! {
                swapbytes::SwapBytes::swap_bytes_mut(&mut self.#ident);
            }
        });

    quote! {
        impl #generics swapbytes::SwapBytes for #ident #generics #where_clause {
            fn swap_bytes_mut(&mut self) {
                #(#swap_bytes_impls)*
            }
        }
    }
    .into()
}
