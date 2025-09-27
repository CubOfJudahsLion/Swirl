//! # AST (Abstract Syntax Tree)
//!
//! Structures for representing the parsed input.
//! The syntax tree for the asset builder is
//! simple just as its syntax is.
//!
//! - Version : 0.1.1
//! - Author  : Alex Feterman
//! - Created : Fri 10/22/2021 07:34 AM
//! - Modified: Thu 06 Jan 2022 02:15:56 PM CST


use std::fmt::{self, Debug, Formatter};
use regex::Regex;
use syn::{self, Ident, LitInt, LitStr, Token, braced, bracketed};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use lazy_static::*;
use super::util::*;


//  Helper function to obtain a PascalCase/CamelCaps identifier.
fn get_camel_caps_ident(input: ParseStream<'_>) -> syn::Result<Ident> {
    lazy_static!{
        static ref TYPE_IDENT_REGEX : Regex =   Regex::new("^[A-Z][a-zA-Z0-9]*$").unwrap();
    }
    let ident   : Ident = input.parse()?;
    let name            = ident.to_string();
    if !TYPE_IDENT_REGEX.is_match(&name) {
        Err(syn::Error::new(ident.span(), "Expected PascalCase identifier."))
    }
    else if name.ends_with("Struct") {
        Err(syn::Error::new(
            ident.span(),
            "No type name should end in 'Struct' as the suffix will be used to generate corresponding struct names."
        ))
    }
    else {
        Ok(ident)
    }
}


//  Helps checking if two identifiers are the same, generating an error result if they do.
fn check_clashing_ident(ident1: &Ident, ident2: &Ident) -> Result<(), (Ident, Ident)> {
    if ident1.to_string() == ident2.to_string() {
        let ident1_copy = Ident::new(&ident1.to_string(), ident1.span());
        let ident2_copy = Ident::new(&ident2.to_string(), ident2.span());
        Err((ident1_copy, ident2_copy))
    }
    else {
        Ok(())
    }
}


/// Files held by media type.
#[derive(Clone)]
pub enum VariantFiles {
    FontFile(LitStr),
    ImageFile(LitStr),
    SoundFiles(LitStr, LitStr),
}

impl Debug for VariantFiles {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use VariantFiles::*;
        f.write_str(
            &match self {
                FontFile(lit)       =>  "FontFile(".to_string()
                                    +   &format!("{:?}", lit.value())
                                    +   ")",
                ImageFile(lit)      =>  "ImageFile(".to_string()
                                    +   &format!("{:?}", lit.value())
                                    +   ")",
                SoundFiles(l1, l2)  =>  "SoundFiles(".to_string()
                                    +   &format!("{:?}", l1.value())
                                    +   &format!("{:?}", l2.value())
                                    +   ")",
            }
        )
    }
}

/// One of the variants inside an enum. This is the simple
/// (terminal) variant, which holds one or two filenames.
#[derive(Clone)]
pub struct Variant {
    name    : Ident,
    files   : VariantFiles,
}

impl Debug for Variant {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "Variant {{ name: {:?}, files: {:?} }}",
            self.name.to_string(),
            self.files
        ))
    }
}

impl Variant {
    #[inline(always)]
    fn check_for_var_ident_clash(&self, ident: &Ident) -> Result<(), (Ident, Ident)> {
        check_clashing_ident(ident, &self.name)
    }

    #[inline(always)]
    pub fn name(&self) -> &Ident { &self.name }

    #[inline(always)]
    pub fn files(&self) -> &VariantFiles { &self.files }
}

impl Parse for Variant {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let name                = input.call(get_camel_caps_ident)?;
        input.parse::<Token![=]>()?;
        let literal : LitStr    = input.parse()?;
        let literal_str         = literal.value();
        use VariantFiles::*;
        if FONT_EXTENSIONS.is_match(&literal_str) {
            Ok(Variant {name, files: FontFile(literal),})
        }
        else if IMG_EXTENSIONS.is_match(&literal_str) {
            Ok(Variant {name, files: ImageFile(literal),})
        }
        else if SND_EXTENSIONS.is_match(&literal_str) {
            let alt_literal = {
                let sans_ext  = literal_str[.. literal_str.len() - 4].to_string();
                let ext       = &literal_str[literal_str.len() - 3 ..];
                fn case_fns(lit: &str) -> Vec<Box<dyn FnMut(&char) -> char>> {
                    #[derive(PartialEq)]
                    enum Case {
                        Upper,
                        Lower,
                        Unknown,
                    }
                    let mut prev_case = Case::Unknown;
                    lit
                        .chars()
                        .map(|c|
                             if c.is_ascii_lowercase() || !c.is_ascii_alphabetic() && prev_case == Case::Lower {
                                 prev_case = Case::Lower;
                                 Box::new(char::to_ascii_lowercase)
                             }
                             else if c.is_ascii_uppercase() || !c.is_ascii_alphabetic() && prev_case == Case::Upper {
                                 prev_case = Case::Upper;
                                 Box::new(char::to_ascii_uppercase)
                             }
                             else {
                                 Box::new(|c: &char| *c) as Box<dyn FnMut(&char) -> char>
                             }
                        )
                        .collect()
                }
                LitStr::new(
                    &(sans_ext + "." + &(&mut case_fns(ext)[..])
                        .iter_mut()
                        .zip((if ext.to_lowercase() == "ogg" { "mp3" } else { "ogg" }).chars())
                        .map(|(f, c)| f(&c))
                      .collect::<String>()
                    ),
                    literal.span()
                )
            };
            Ok(Variant {name, files: SoundFiles(literal, alt_literal),})
        }
        else {
            Err(syn::Error::new(
                literal.span(),
                "Accepted extensions are: woff, jpg, jpeg, png, svg, ogg and mp3."
            ))
        }
    }
}


/// An enumeration, full of variants or other enums. This is the root of our AST.
#[derive(Debug)]
pub struct Enum {
    enum_name   : Ident,
    set         : Vec<EnumOrVariant>,
}

impl Enum {
    fn check_for_var_ident_clash(&self) -> Result<(), (Ident, Ident)> {
        for elem_idx in 0 .. self.set.len() {
            let elem_name = &self.set[elem_idx].get_var_ident();
            for other_elem in &self.set[elem_idx + 1 ..] {
                other_elem.check_for_var_ident_clash(elem_name)?;
            }
        }
        Ok(())
    }

    fn check_for_enum_ident_clash(&self, ident: Option<&Ident>) -> Result<(), (Ident, Ident)> {
        if let &Some(ident) = &ident {
            check_clashing_ident(&self.enum_name, ident)?;
            for elem in &self.set {
                elem.check_for_enum_ident_clash(ident)?;
            }
        }
        for elem in &self.set {
            elem.check_for_enum_ident_clash(&self.enum_name)?;
        }
        Ok(())
    }

    pub fn check_for_ident_clash(&self) -> Result<(), (Ident, Ident)> {
        self.check_for_var_ident_clash()?;
        self.check_for_enum_ident_clash(None)
    }

    #[inline(always)]
    pub fn enum_name(&self) -> &Ident { &self.enum_name }

    #[inline(always)]
    pub fn set(&self) -> &[EnumOrVariant] { &self.set }
}

impl Parse for Enum {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let enum_name       = input.call(get_camel_caps_ident)?;
        //  After the name, what follows is Recursive Descent Parsing.
        let sub_variant_stream;
        braced!(sub_variant_stream in input);
        let inside_braces   = sub_variant_stream.span();
        let sub_variants    = Punctuated::<EnumOrVariant, Token![,]>::parse_terminated(&sub_variant_stream)?;
        if sub_variants.is_empty() {
            Err(syn::Error::new(inside_braces, "No variants inside enum."))
        }
        else {
            Ok(Enum {
                enum_name,
                set : sub_variants.into_iter().collect::<Vec<_>>(),
            })
        }
    }
}


#[derive(Debug)]
pub struct VarEnum {
    var_name: Ident,
    enu     : Enum,
}

impl VarEnum {
    fn check_for_var_ident_clash(&self, ident: &Ident) -> Result<(), (Ident, Ident)> {
        check_clashing_ident(ident, &self.var_name)?;
        self.enu.check_for_ident_clash()
    }

    fn check_for_enum_ident_clash(&self, ident: &Ident) -> Result<(), (Ident, Ident)> {
        self.enu.check_for_enum_ident_clash(Some(ident))
    }

    #[inline(always)]
    pub fn var_name(&self) -> &Ident { &self.var_name }
    
    #[inline(always)]
    pub fn enu(&self) -> &Enum { &self.enu }
}

impl Parse for VarEnum {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let var_name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let enu             = input.parse::<Enum>()?;
        Ok(VarEnum {
            var_name,
            enu,
        })
    }
}


/// A ranged enumeration. These are entries with a `VarName : EnumName "prefix" [x ..= y : n] "postfix"`
/// pattern. Filenames are constructed starting with the prefix, then attaching as number in the range [x,y]
/// (zero-padded to `n` digits if present) and then the postfix.
#[derive(Debug)]
pub struct RangedEnum {
    var_name        : Ident,
    enum_name       : Ident,
    file_prefix     : String,
    min             : usize,
    max             : usize,
    zero_padding    : usize,
    file_postfix    : String,
}

impl RangedEnum {
    fn check_for_var_ident_clash(&self, ident: &Ident) -> Result<(), (Ident, Ident)> {
        check_clashing_ident(ident, &self.var_name)
    }

    fn check_for_enum_ident_clash(&self, ident: &Ident) -> Result<(), (Ident, Ident)> {
        check_clashing_ident(&self.enum_name, ident)
    }
    
    #[inline(always)]
    pub fn var_name(&self) -> &Ident { &self.var_name }
    
    #[inline(always)]
    pub fn enum_name(&self) -> &Ident { &self.enum_name }
    
    #[inline(always)]
    pub fn file_prefix(&self) -> &str { &self.file_prefix }
    
    #[inline(always)]
    pub fn min(&self) -> usize { self.min }
    
    #[inline(always)]
    pub fn max(&self) -> usize { self.max }
    
    #[inline(always)]
    pub fn zero_padding(&self) -> usize { self.zero_padding }
    
    #[inline(always)]
    pub fn file_postfix(&self) -> &str { &self.file_postfix }
}

impl Parse for RangedEnum {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let var_name            = input.call(get_camel_caps_ident)?;
        input.parse::<Token![:]>()?;
        let enum_name           = input.call(get_camel_caps_ident)?;
        input.parse::<Token![=]>()?;
        let file_prefix_lit     = input.parse::<LitStr>()?;
        let file_prefix         = file_prefix_lit.value();
        let range_input;
        bracketed!(range_input in input);
            let (min, min_span)     = {
                if range_input.peek(Token![..]) || range_input.peek(Token![..=]) {
                    (0, range_input.span())
                }
                else {
                    let min_lit = range_input.parse::<LitInt>()?;
                    ( min_lit.base10_parse::<usize>()?
                    , min_lit.span()
                    )
                }
            };
            let max                 = {
                let inclusive_range =
                    if range_input.peek(Token![..=]) {
                        let _ = range_input.parse::<Token![..=]>();
                        true
                    }
                    else {
                        range_input.parse::<Token![..]>()?;
                        false
                    };
                let max_lit         = range_input.parse::<LitInt>()?;
                max_lit.base10_parse::<usize>()? - if inclusive_range { 0 } else { 1 }
            };
            let zero_padding        =
                if range_input.peek(Token![:]) {
                    let _ = range_input.parse::<Token![:]>();
                    let parsed_padding  = range_input.parse::<LitInt>()?;
                    parsed_padding.base10_parse::<usize>()?
                }
                else {
                    0
                };
        let file_postfix_lit    = input.parse::<LitStr>()?;
        let file_postfix        = file_postfix_lit.value();
        //  Now we validate the parsed tokens semantically.
        if !range_input.is_empty() {
            Err(syn::Error::new(range_input.span(), "Extra tokens inside range brackets."))
        }
        else if min > max {
            Err(syn::Error::new(min_span, "File range enum should start with the lesser number."))
        }
        else if !IMG_EXTENSIONS.is_match(&file_postfix) {
            Err(syn::Error::new(
                file_postfix_lit.span(),
                "File type in a range must be an image type (svg, png, jpg.)"
            ))
        }
        else {
            Ok(RangedEnum {
                var_name,
                enum_name,
                file_prefix,
                min,
                max,
                zero_padding,
                file_postfix,
            })
        }
    }
}


/// Inside an enumeration, there can be other enums, range enums or simple
/// variants.  We need a type to model all of these possibilities.
#[derive(Debug)]
pub enum EnumOrVariant {
    Var(Variant),
    VEn(VarEnum),
    Rng(RangedEnum),
}

impl EnumOrVariant {
    fn check_for_var_ident_clash(&self, ident: &Ident) -> Result<(), (Ident, Ident)> {
        use EnumOrVariant::*;
        match self {
            &Var(ref var)   => var.check_for_var_ident_clash(ident),
            &VEn(ref ven)   => ven.check_for_var_ident_clash(ident),
            &Rng(ref re)    => re.check_for_var_ident_clash(ident),
        }
    }

    fn check_for_enum_ident_clash(&self, ident: &Ident) -> Result<(), (Ident, Ident)> {
        use EnumOrVariant::*;
        match self {
            &VEn(ref ven)   => ven.check_for_enum_ident_clash(ident),
            &Rng(ref re)    => re.check_for_enum_ident_clash(ident),
            &Var(_)         => Ok(()),
        }
    }

    fn get_var_ident(&self) -> &Ident {
        use EnumOrVariant::*;
        match self {
            &Var(ref var)   => &var.name,
            &VEn(ref ven)   => &ven.var_name,
            &Rng(ref re)    => &re.var_name,
        }
    }
}

impl Parse for EnumOrVariant {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        //  Peek and leave the heavy lifting to the appropriate
        //  parser for the internal type.
        if input.peek2(Token![:]) {
            let input_clone = input.fork();
            input_clone.parse::<Ident>()?;
            if input_clone.peek3(Token![=]) {
                Ok(EnumOrVariant::Rng(input.call(RangedEnum::parse)?))
            }
            else {
                Ok(EnumOrVariant::VEn(input.call(VarEnum::parse)?))
            }
        }
        else if input.peek2(Token![=]) {
            Ok(EnumOrVariant::Var(input.call(Variant::parse)?))
        }
        else {
            Err(input.error("Input does not resemble a variant or an enumeration."))
        }
    }
}
