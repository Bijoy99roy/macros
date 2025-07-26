use proc_macro::TokenStream;

use quote::quote;
use syn::{
    Data, DeriveInput, Fields, Ident, LitInt, Token, braced, bracketed,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::{Brace, Bracket, Paren},
};

#[derive(Debug)]
struct Seed {
    values: Vec<u8>,
}

impl Parse for Seed {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // check for seed= prefix
        let ident = input.call(Ident::parse_any)?;
        if ident != "seed" {
            return Err(input.error("Expected 'seed'"));
        }
        input.parse::<Token![=]>()?;
        eprintln!("#############{:#?}#############", input);
        // parse the value inside [], {} or ()
        let value: Vec<u8> = if input.peek(Bracket) {
            let content;
            bracketed!(content in input);
            eprintln!("-----------------{:#?}------------------------", content);
            parse_comma_seperated_values(&content)?
        } else if input.peek(Paren) {
            let content;
            parenthesized!(content in input);
            eprintln!("-----------------{:#?}------------------------", content);
            parse_comma_seperated_values(&content)?
        } else if input.peek(Brace) {
            let content;
            braced!(content in input);
            eprintln!("-----------------{:#?}------------------------", content);
            parse_comma_seperated_values(&content)?
        } else {
            println!("Here");
            return Err(input.error("Expected '[', '{', or '(' after seed="));
        };

        Ok(Seed { values: value })
    }
}

fn parse_comma_seperated_values(input: ParseStream) -> syn::parse::Result<Vec<u8>> {
    eprintln!("{:#?}", input);
    let punctuated: Punctuated<LitInt, Token![,]> = Punctuated::parse_terminated(input)?;

    let value = punctuated
        .into_iter()
        .map(|lit| {
            lit.base10_parse::<u8>()
                .map_err(|_| syn::Error::new(lit.span(), "Expected u8 integrer"))
        })
        .collect::<syn::parse::Result<Vec<u8>>>()?;
    eprintln!("{:#?}", value);
    Ok(value)
}
#[proc_macro_derive(AnyBracket, attributes(attrib))]
pub fn allow_any_bracket(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(named_fields) => &named_fields.named,
            _ => panic!("Only named fields supported"),
        },
        _ => panic!("Only structs supported"),
    };

    let field_checks = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let seed_attr = match field.attrs.iter().find(|attr| attr.path().is_ident("attrib")) {
            Some(attr) => attr,
            None => return quote! {}, // Skip fields without attrib
        };
        let seed: Seed = match seed_attr.parse_args() {
            Ok(seed) => seed,
            Err(e) => {
                let error_msg = e.to_string();
                return quote! { compile_error!(#error_msg); };
            },
        };
        let values = seed.values;

        quote! {
            println!("Field {} has seed values: {:?}", stringify!(#field_name), vec![#(#values),*]);
        }
    }).collect::<Vec<_>>();

    let expanded = quote! {
        impl #name {
            pub fn check_seeds(&self) {
                #(#field_checks)*
            }
        }
    };
    eprintln!("Generated code: {}", expanded);
    TokenStream::from(expanded)
}
