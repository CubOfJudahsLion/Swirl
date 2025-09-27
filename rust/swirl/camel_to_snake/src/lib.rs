//! Module  :   camelTo_snake
//! Purpose :   Procedural macro that changes camelCase into snake_case
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue 03/29/2022 03:03 PM
//! Modified:   ~


use syn::{parse_macro_input, Ident};
use proc_macro::{TokenStream, TokenTree, Span};
use regex::{Regex, Captures};


#[proc_macro]
pub fn camel_to_snake(ts: TokenStream) -> TokenStream {
    let id_camel    = parse_macro_input!(ts as Ident).to_string();
    let re          = Regex::new("([a-z])([A-Z])").unwrap();
    let str_snake   = re.replace_all(
        &id_camel,
        |caps: &Captures| format!("{}_{}", &caps[1], &caps[2].to_lowercase())
    );
    TokenStream::from(TokenTree::Ident(proc_macro::Ident::new(&str_snake, Span::call_site())))
}
