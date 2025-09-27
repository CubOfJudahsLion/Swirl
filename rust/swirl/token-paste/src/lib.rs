//! Module  :   Token Paste
//! Purpose :   A minimal procedural macro to paste tokens together
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue 10/12/2021 08:16 AM
//! Modified:   Tue 10/12/2021 11:21 AM


use core::iter::Extend;
use proc_macro::*;
//use proc_macro2::{self, Span};
//use quote::ToTokens;
//use syn::{custom_punctuation, parse_macro_input, Ident, Result};
//use syn::parse::{Parse, ParseStream, Nothing};
//
//
//custom_punctuation!{AnglePlus, <+}
//custom_punctuation!{PlusAngle, +>}
//
//struct PastingSingleRequest(Vec<Ident>);
//
//impl Parse for PastingSingleRequest {
//    fn parse(input: ParseStream) -> Result<Self> {
//        let mut v = Vec::<Ident>::with_capacity(32);
//        let _ : AnglePlus = input.parse()?;
//        loop {
//            let ident: Ident = input.parse()?;
//            v.push(ident);
//            if input.is_empty() || input.peek(PlusAngle) {
//                break
//            }
//        }
//        let _ : PlusAngle = input.parse()?;
//        Ok(PastingSingleRequest(v))
//    }
//}
//
//struct PastedTree(proc_macro2::TokenStream);
//
//impl Parse for PastedTree {
//    fn parse(input: ParseStream) -> Result<Self> {
//        
//    }
//}


fn map_token_tree(tree: TokenTree) -> TokenTree {
    match tree {
        TokenTree::Group(grp) =>
            TokenTree::Group(
                Group::new(
                    grp.delimiter(),
                    map_token_stream(grp.stream())
                )
            ),
    }
}

fn map_token_stream(stream: TokenStream) -> TokenStream {
    if stream.is_empty() {
        stream
    }
    else {
        let mut result  = TokenStream::new();
        let mut iter    = stream.into_iter();
        loop {
            let next_tt = { iter.next() }
            match next_tt {
                Some(TokenTree::Punct(punct))
                    if      punct.as_char() == '<'
                        &&  punct.spacing() == Spacing::Joint   => {
                },
                _                                               => {
                    result.extend(iter.take(1));
                }
            }
        }
        result
    }
}

#[proc_macro_attribute]
pub fn token_paste(args: TokenStream, tokens: TokenStream) -> TokenStream {
    map_token_stream(tokens)
}
