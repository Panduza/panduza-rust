


extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};


// il faut aussi utiliser les proc attributes pour les attributs et non pas derive


#[proc_macro_derive(HelloWorld)]
pub fn hello_world(input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as DeriveInput);


    let name = input.ident;

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        
        impl HelloWorld for #name {
            fn hello_world() {
                println!("Hello, World! My name is {}", stringify!(#name));
            }
        }
        
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

// fn impl_hello_world(ast: &syn::DeriveInput) -> quote::ToTokens {
//     let name = &ast.ident;
//     quote! {
//         impl HelloWorld for #name {
//             fn hello_world() {
//                 println!("Hello, World! My name is {}", stringify!(#name));
//             }
//         }
//     }
// }
