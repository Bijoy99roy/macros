use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Parse input into token stream, here AST(Abstract Syntax Tree)
    let ast = syn::parse(input).unwrap();

    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen_code = quote! {
        impl HelloMacro for #name {
            fn hello(){
                println!("Hello, I'm {}", stringify!(#name));
            }
        }
    };
    gen_code.into()
}
