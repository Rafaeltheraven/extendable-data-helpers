use syn::{Variant, self, DeriveInput};
use proc_macro2::TokenStream;
use quote::quote;

fn combine_enum_variants<'input>(enum_a: &'input syn::DataEnum, enum_b: &'input syn::DataEnum) -> std::iter::Chain<syn::punctuated::Iter<'input, Variant>, syn::punctuated::Iter<'input, Variant>> {
    let iter_a = enum_a.variants.iter();
    let iter_b = enum_b.variants.iter();
    iter_a.chain(iter_b)
}

fn _combine_enums<'input>(enum_a: &'input syn::DataEnum, enum_b: &'input syn::DataEnum, name: syn::Ident, attrs: Vec<syn::Attribute>) -> TokenStream {
    let combined_tokens = combine_enum_variants(enum_a, enum_b);
    let final_tokens = quote! {
        #(#attrs)*
        enum #name {
            #(#combined_tokens),*
        }
    };

    TokenStream::from(final_tokens)
}

/// Combines two enums into a single enum using TokenStreams.
///
/// Used internally by extendable-enums to construct a tokenstream which has the contents
/// of two enums glued together. The name of the enum in input_b will be the name of the "new" enum.
///
/// Technically this could probably be extended to structs as well, but I haven't looked into it.
pub fn combine_enums(input_a: TokenStream, input_b: TokenStream) -> TokenStream {
    let ast_a = syn::parse2::<DeriveInput>(input_a).unwrap();
    let ast_b = syn::parse2::<DeriveInput>(input_b).unwrap();
    match (&ast_a.data, &ast_b.data) {
        (syn::Data::Enum(enum_a), syn::Data::Enum(enum_b)) => _combine_enums(&enum_a, &enum_b, ast_b.ident, ast_a.attrs.into_iter().chain(ast_b.attrs.into_iter()).collect()),
        _ => panic!("Can only combine enums!")
    }
}