use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_attribute]
pub fn attribute_auto_display(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("Only named fields supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let field_names = fields.iter().map(|f| f.ident.as_ref().unwrap());
    let field_strings = field_names.clone().map(|name| name.to_string());

    let gen_code = quote! {
        #input

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    concat!(stringify!(#name), " {{ ", #( #field_strings, ": {}, ", )* "}}"),
                    #( self.#field_names ),*
                )
            }
        }
    };

    gen_code.into()
}
