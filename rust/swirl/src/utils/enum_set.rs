//! Module  :   Enumerated Set
//! Purpose :   `EnumSet` is An ordered `enum` with macro-automated string
//!             representation and `Copy`. These `enum`s can be used as
//!             indexes in the accompanying `EnumSetIndexIndexed` type.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Thu Mar 11 11:55:43 AM CST 2021
//! Modified:   Tue 03/22/2022 02:57 PM


use std::fmt::Display;
use std::hash::Hash;
use std::ops::{Index, IndexMut};
use std::slice::Iter;
use std::marker::PhantomData;


/// An enumerable set, built as an enum with an ordered list. Its requisites also
/// state that it should define an order, and be convertible into an usize (i.e.,
/// a number capable of indexing) and produce a constant string to represent each
/// variant.
pub trait EnumSet   : Ord
                    + Eq
                    + Hash
                    + Copy
                    + Into<usize>
                    + Display
                    + 'static
{
    const ORDERED_VARIANTS  : &'static [Self];
    const VARIANT_COUNT     : usize = Self::ORDERED_VARIANTS.len();
    fn ordered_iter() -> Iter<'static, Self> { Self::ORDERED_VARIANTS.into_iter() }
}

/// This macro does the job of fulfilling creating an enum type that fulfills all
/// of the ```EnumSet``` conditions and implements the trait. Usage:
/// 
///     make_enum_set!{
///         Currency ::=
///             Dollar  => "$",
///             Euro    => "€",
///             Pound   => "£",
///             Yen     => "¥",
///     }
///
/// This will define the enum `Currency` with the enumerated variants, counts,
/// an ordered list (increasing in order of appearance) and respective conversions
/// (to `usize` and `&'static str`) is implemented automatically.
/// If instead of `=>` the variants and their string mappings are connected by
/// `<=>`, additional reverse mappings (`TryFrom`) are also implemented.
/// The variants are considered to be a *full* order, hashable and copyable. Do
/// note that they're represented as `usize` to minimize conversion work.
#[macro_export]
macro_rules! make_enum_set {
    ($enum_name:ident ::=
        $($variant:ident => $str:expr),+ $(,)?
    ) =>
    {
        #[repr(usize)]
        #[wasm_bindgen::prelude::wasm_bindgen]
        #[derive(std::cmp::PartialEq, std::cmp::PartialOrd, std::cmp::Eq, std::cmp::Ord,
                 std::hash::Hash, std::clone::Clone, std::marker::Copy, std::fmt::Debug)]
        pub enum $enum_name {
            $($variant,)+
        }

        impl std::convert::From<$enum_name> for usize {
            fn from(e: $enum_name) -> usize {
                e as usize
            }
        }

        impl std::convert::From<$enum_name> for &'static str {
            fn from(e: $enum_name) -> Self {
                match e {
                    $($enum_name::$variant => $str,)+
                }
            }
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(<&'static str as std::convert::From<$enum_name>>::from(*self))
            }
        }

        impl crate::utils::EnumSet for $enum_name {
            const ORDERED_VARIANTS: &'static [$enum_name] = &[
                $($enum_name::$variant,)+
            ];
        }
    };
    ($enum_name:ident ::=
        $($variant:ident <=> $str:expr),+ $(,)?
    ) =>
    {
        make_enum_set!{
            $enum_name ::=
                $($variant => $str,)+
        }

        impl std::convert::TryFrom<&str> for $enum_name {
            type Error = ();

            fn try_from(val: &str) -> std::result::Result<Self, Self::Error> {
                use $enum_name::*;
                match val {
                    $($str => Ok($variant),)+
                    _ => Err(()),
                }
            }
        }
    };
}

//  Export the macro from this module too.
pub use crate::make_enum_set;


/// An array that is indexed by an `EnumSet`. Since they implement a full
/// order, a linear array suffices to index all variants in constant time.
#[derive(Debug)]
pub struct EnumSetIndexed<E, T>(Box<[T]>, PhantomData<E>,);

impl<E, T> EnumSetIndexed<E, T>
    where   E:  EnumSet
{
    pub fn new_mapped<F: FnMut(&E) -> T>(f: &mut F) -> EnumSetIndexed<E, T> {
        EnumSetIndexed::<E, T>(
            E::ordered_iter()
                .map(|e| f(e))
                .collect::<Vec<T>>()
                .into_boxed_slice(),
            PhantomData,
        )
    }

    #[allow(dead_code)]
    pub fn try_get<'a>(&'a self, idx: usize) -> Option<&'a T> {
        if idx >= self.0.len() {
            None
        }
        else {
            Some(&self.0[idx])
        }
    }
}

impl<E, T> Index<E> for EnumSetIndexed<E, T>
    where   E       : EnumSet,
            usize   : From<E>,
{
    type Output = T;

    fn index(&self, e: E) -> &Self::Output {
        let &EnumSetIndexed(ref contents, _) = self;
        &contents[<usize as From<E>>::from(e)]
    }
}

impl<E, T> IndexMut<E> for EnumSetIndexed<E, T>
    where   E       : EnumSet,
            usize   : From<E>
{
    fn index_mut(&mut self, e: E) -> &mut Self::Output {
        let &mut EnumSetIndexed(ref mut contents, _) = self;
        &mut contents[<usize as From<E>>::from(e)]
    }
}
