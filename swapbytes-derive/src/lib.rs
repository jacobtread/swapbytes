use darling::FromAttributes;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DataEnum, DataStruct, DeriveInput};

#[derive(Debug, FromAttributes)]
#[darling(attributes(sb), forward_attrs(allow, doc, cfg))]
struct FieldAttr {
    #[darling(default)]
    skip: bool,
}

#[proc_macro_derive(SwapBytes, attributes(sb))]
pub fn derive_swap_bytes(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    match &input.data {
        syn::Data::Struct(data) => impl_struct(&input, data),
        syn::Data::Enum(data) => impl_enum(&input, data),
        _ => panic!("SwapBytesMut only supported on structs and enums"),
    }
}

/// Implements [`SwapBytes`] for a struct
fn impl_struct(input: &DeriveInput, data: &DataStruct) -> TokenStream {
    let ident = &input.ident;
    let generics = &input.generics;
    let where_clause = generics.where_clause.as_ref();

    // Generate `swap_bytes_mut` calls for all the non-skipped fields
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

/// Implements [`SwapBytes`] for a repr enum
fn impl_enum(input: &DeriveInput, _data: &DataEnum) -> TokenStream {
    // Extract the repr type from the enum attributes
    let repr = input
        .attrs
        .iter()
        .find_map(|attr| match &attr.meta {
            syn::Meta::List(list) if list.path.is_ident("repr") => Some(&list.tokens),
            _ => None,
        })
        .expect("Enum missing repr type");

    let ident = &input.ident;

    // Impl casts the enum to its repr type, swaps its order then transmutes back
    quote! {
        impl swapbytes::SwapBytes for #ident {
            fn swap_bytes_mut(&mut self) {
                let mut value: #repr = *self as #repr;
                value.swap_bytes_mut();
                *self = unsafe { std::mem::transmute(value) };
            }
        }
    }
    .into()
}
