extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{ parse_macro_input, LitStr };
use quote::quote;

use bfrunner::run_to_string;

#[proc_macro]
pub fn bf(input: TokenStream) -> TokenStream {
    println!("⚠️ Warning: Brainfuck code is being executed at compile-time!");
    let input_lit = parse_macro_input!(input as LitStr);
    let source = input_lit.value();

    let output = run_to_string(&source, &mut std::io::empty()).unwrap_or_else(|_|
        String::from("<compile-time error>")
    );

    let expanded = quote! {
        #output
    };

    expanded.into()
}
