use proc_macro::TokenStream;
use quote::quote;

use syn::{Data, DeriveInput, Fields, parse_macro_input};
#[proc_macro_attribute]
pub fn prepend_todo(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;
    let vis = &input.vis;
    let generics = &input.generics;

    let data = match input.data {
        Data::Struct(data_struct) => data_struct,
        _ => panic!("`prepend_todo` can only be applied to structs"),
    };

    let fields = match data.fields {
        Fields::Named(ref fields_named) => &fields_named.named,
        _ => panic!("`prepend_todo` only supports structs with named fields"),
    };

    let new_fields = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        let pascal = to_pascal_case(&ident.to_string());
        let renamed = format!("TodoApp{}", pascal);
        quote! {
            #[serde(rename = #renamed)]
            #ident: #ty
        }
    });

    let gen_code = quote! {
        #[derive(serde::Serialize)]
        #vis struct #struct_name #generics {
            #(#new_fields,)*
        }
    };
    TokenStream::from(gen_code)
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join("")
}
