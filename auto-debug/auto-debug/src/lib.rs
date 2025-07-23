use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(AutoDebug, attributes(debug))]
pub fn attribute_auto_debug(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let output = impl_debug(&input);
    quote! {
        #output
    }
    .into()
}

fn impl_debug(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data_structs) => match &data_structs.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let mut field_calls = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_string = field_name.to_string();

        let skip = field.attrs.iter().any(|attr| is_debug_skip(attr));
        if !skip {
            field_calls.push(quote! {
                .field(#field_string, &self.#field_name)
            });
        }
    }

    quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!(#name))
                #(#field_calls)*
                .finish()
            }
        }
    }
}

fn is_debug_skip(attr: &Attribute) -> bool {
    let mut skip = false;
    if attr.path().is_ident("debug") {
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                skip = true;
            }
            Ok(())
        });
    }
    skip
}
