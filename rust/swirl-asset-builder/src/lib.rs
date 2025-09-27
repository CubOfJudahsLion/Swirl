//! # Structured Enumerated Set
//!
//! Macro facilities to easy asset declaration, retrieval and
//! assembly into a structured type.
//!
//! - *Version* : 0.1.1
//! - *Author*  : Alex Feterman
//! - *Created* : Thu 10/21/2021 04:35 PM
//! - *Modified*: Sun 02 Jan 2022 11:13:07 PM CST


use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, Span};
use syn::{self, parse2};
use quote::quote;

mod common;
mod ast;
mod util;
mod ir;
mod printer;


/// Delegates parsing job as this function uses `proc_macro2::TokenStream`s.
/// This makes it easier to test parsing during runtime.
fn actual(tokens: TokenStream2) -> TokenStream2 {
    match parse2::<ast::Enum>(tokens)
        .and_then(|repr| repr
            .check_for_ident_clash()
            .map_err(|(ident1, ident2)| {
                let mut err1    = syn::Error::new(
                    ident1.span(),
                    "Identifier found here:"
                );
                let err2        = syn::Error::new(
                    ident2.span(),
                    "is repeated here. Repeating names is not allowed."
                );
                err1.combine(err2);
                err1
            })
            .map(|()| repr)
        )
        .and_then(|repr| ir::ast_to_ir(repr)
            .map_err(|_|
                syn::Error::new(
                    Span::call_site(),
                    "Internal error: error transforming syntax tree."
                )
            )
        )
    {
        Ok(repr)    => {
            let output_str = format!("{}", printer::pretty_print(repr));
            quote!{let temp_output = #output_str;}
        },
        Err(err)    => {
            err.into_compile_error()
        }
    }
}


/// Facilitates building `EnumSet`s and `struct`s to match a hierarchy
/// of assets, and automatically produces a loader mechanism.
///
/// ## Syntax
///
/// ```ignore
///     build_assets!{
///         OuterEnumName {
///             SimpleVariant = "filepath.jpg",
///             RangedVariant : EnumName1 = "prefix" [1 .. 25 : 4] "postfix.png",
///             ComplexVariant : EnumName2 {
///                 InnerSimpleVariant1 = "another-filepath1.jpg",
///                 InnerSimpleVariant2 = "another-filepath2.jpeg",
///                 InnerSimpleVariant2 = "another-filepath2.woff",
///                 AnotherComplexVariant: EnumName3 {
///                 },
///                 ...
///             },
///             ...
///         }
///     }
/// ```
/// The syntax is almost trivial. Every item starts with a *CamelCaps*
/// (aka *PascalCase*) identifier. Then follows a definition body that
/// varies by type.
///
/// ### Simple definitions
/// The simplest item definitions are those followed by an equal sign
/// and a **valid** filename (i.e., it must have a loadable extension,
/// such as `.jpg` or `.woff`.)
/// ```ignore
///     SimpleDefinitionButton = "the-button.png"
/// ```
/// If the extension of the filename happens to be `.ogg` or `.mp3`,
/// it will be identified as a sound and generate code to load
/// a sound using both extensions.
///
/// ### Ranged definitions
/// These specify a range and an optional zero-padding.  These are
/// reserved for image sequences, and will be converted into
/// `SpriteMovie`s. Both range operators (`..`, `..=`) can be used
/// to specify range.
/// ```ignore
///     RangedDefinitionForMovie : EnumName = "prefix-name" [0 ..= 8] ".jpg"
/// ```
/// For zero-padded numbering, it's also possible to express the total
/// number of digits:
/// ```ignore
///     RangedDefinitionForMovie : EnumName = "prefix-name" [ .. 1000 : 3] "-suffix.jpg"
/// ```
/// As shown above, the initial number can be omitted if it is zero.
///
/// ### Complex definitions
/// These contain other definition. The leading identifier is followed
/// by an opening brace (`{`), a list of definitions (simple, ranged
/// or complex) separated by commas, and the closing brace (`}` .)
/// ```ignore
///     ComplexDefinition: EnumName {
///         SimpleDefinitionButton = "the-button.png",
///         AnotherComplexDefinition : EnumName2 {
///             ...
///         },
///     }
/// ```
/// In a very Rust-like way, the last definition may be followed by
/// a trailing comma. There's no prefixed limit in the nesting level
/// of complex definitions,
#[proc_macro]
pub fn build_assets(tokens: TokenStream) -> TokenStream {
    actual(tokens.into()).into()
}
