use syn::token::{Comma, Lt, Gt, Where};
use syn::{Variant, self, DeriveInput, Generics, GenericParam, WhereClause, WherePredicate};
use syn::punctuated::{Iter, Punctuated};
use proc_macro2::TokenStream;
use quote::quote;

fn combine_enum_variants<'input>(enum_a: &'input syn::DataEnum, enum_b: &'input syn::DataEnum) -> std::iter::Chain<Iter<'input, Variant>, Iter<'input, Variant>> {
    let iter_a = enum_a.variants.iter();
    let iter_b = enum_b.variants.iter();
    iter_a.chain(iter_b)
}

fn combine_wheres(where_a: WhereClause, where_b: WhereClause) -> WhereClause {
    let pred_a = where_a.predicates.into_iter();
    let pred_b = where_b.predicates.into_iter();
    let combined: Punctuated<WherePredicate, Comma> = Punctuated::from_iter(pred_a.chain(pred_b));
    WhereClause { 
        where_token: Where::default(), 
        predicates: combined,
    }
}

fn combine_generics(input_a: Generics, input_b: Generics) -> Generics {
    let params_a = input_a.params.clone().into_pairs();
    let params_b = input_b.params.clone().into_pairs();
    let combined = params_a.chain(params_b);
    let params_c: Punctuated<GenericParam, Comma> = Punctuated::from_iter(combined);
    let where_c: Option<WhereClause> = match (input_a.where_clause, input_b.where_clause) {
        (None, None) => None,
        (None, Some(where_b)) => Some(where_b),
        (Some(where_a), None) => Some(where_a),
        (Some(where_a), Some(where_b)) => Some(combine_wheres(where_a, where_b)),
    };
    Generics { 
        lt_token: Some(Lt::default()), 
        params: params_c, 
        gt_token: Some(Gt::default()), 
        where_clause: where_c
    }
}

fn _combine_enums<'input>(enum_a: &'input syn::DataEnum, enum_b: &'input syn::DataEnum, visibility: &syn::Visibility, gens: Generics, name: syn::Ident, attrs: Vec<syn::Attribute>) -> TokenStream {
    let combined_tokens = combine_enum_variants(enum_a, enum_b);
    let final_tokens = quote! {
        #(#attrs)*
        #visibility enum #name #gens {
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
    let vis_b = &ast_b.vis;
    let generics = combine_generics(ast_a.generics, ast_b.generics);
    match (&ast_a.data, &ast_b.data) {
        (syn::Data::Enum(enum_a), syn::Data::Enum(enum_b)) => _combine_enums(&enum_a, &enum_b, vis_b, generics, ast_b.ident, ast_a.attrs.into_iter().chain(ast_b.attrs.into_iter()).collect()),
        _ => panic!("Can only combine enums!")
    }
}