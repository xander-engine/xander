#![feature(proc_macro_span, proc_macro_diagnostic)]

use std::sync::Mutex;

use lazy_static::lazy_static;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use taxonomy::Namespace;

mod taxonomy;


lazy_static! {
    static ref NAMESPACE : Mutex<Option<Namespace>> = Mutex::new(None);
}

#[proc_macro]
pub fn namespace(input : TokenStream) -> TokenStream {
    if input.is_empty() {
        let t = &*NAMESPACE.lock().unwrap();
        match t {
            None => {
                return quote! {
                    compile_error!("Namespace not defined here!");
                }.into();
            },
            Some(v) => {
                let id = v.id();
                return quote! {
                    #id
                }.into()
            }
        }
    }

    let mut t = NAMESPACE.lock().unwrap();

    match *t {
        None => {
            let n = parse_macro_input!(input as Namespace);

            *t = Some(n);

            quote! {}.into()
        },
        Some(ref s) => {
            let msg = format!(
                "Namespace already declared for this crate @ {}",
                s.source_loc()
            );
            quote! {
                compile_error!(#msg)
            }.into()
        }
    }


}