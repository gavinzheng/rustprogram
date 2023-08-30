use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn};

#[proc_macro_derive(MyMacro)]
pub fn my_macro_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!{input};

    let gen = quote! {
        impl MyTrait for #ident {
            fn hello() {
                println!("Hello from {}", stringify!(#ident));
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    println!("{} defined", input.sig.ident);
    println!("Args received: {}", _attr.to_string());
    TokenStream::from(quote!(#input))
}

#[proc_macro]
pub fn print_and_replace(input: TokenStream) -> TokenStream {
    println!("Inputs: {}", input.to_string());
    "fn add(a:i32, b:i32) -> i32 { a+ b }".parse().unwrap()
}