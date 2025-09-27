//! Module  :   Int and Float
//! Purpose :   A library to establish numeric types and differentiate
//!             between integral and floating-point types.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue Jul 20 01:06:45 PM CST 2021
//! Modified:   Sat 06/11/2022 06:40 PM


use std::convert::TryFrom;
use std::num::FpCategory;
use std::ops::{
    Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign
};


#[repr(u32)]
pub enum NumericBits {
    B8     =   8,
    B16    =  16,
    B32    =  32,
    B64    =  64,
    B128   = 128,
}

impl TryFrom<u32> for NumericBits {
    type Error = ();

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        use NumericBits::*;
        Ok(match val {
              8 => B8,
             16 => B16,
             32 => B32,
             64 => B64,
            128 => B128,
            _   => Err(())?,
        })
    }
}


pub trait Numeric   :   Add<Output=Self> + Sub<Output=Self> 
                    +   Mul<Output=Self> + Div<Output=Self> + Rem<Output=Self>
                    +   AddAssign + SubAssign + MulAssign + DivAssign + RemAssign
                    +   PartialOrd + PartialEq
                    +   Clone + Copy + Default
{
    const MIN: Self;
    const MAX: Self;
}

macro_rules! impl_numeric {
    ($($type:ident),+) => {
        $(
            impl Numeric for $type {
                const MIN: $type = $type::MIN;
                const MAX: $type = $type::MAX;
            }
        )+
    }
}

impl_numeric!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64);


pub trait NumSignedness: Numeric {
    const SIGNED: bool;
}

pub trait Signed: Numeric + NumSignedness + Neg<Output=Self> {
    fn abs(self) -> Self;
}

macro_rules! impl_signed {
    (@cond_signed true $type:ty)        => {
        impl Signed for $type {
            #[inline(always)]
            fn abs(self) -> Self { <$type>::abs(self) }
        }
    };
    (@cond_signed false $type:ty)       => {
    };
    ($signed:ident for $($type:ty),+)   => {
        $(
            impl NumSignedness for $type { const SIGNED: bool = $signed; }
            impl_signed!{@cond_signed $signed $type}
        )+
    };
}


impl_signed!(false for u8, u16, u32, u64, u128, usize);
impl_signed!(true for i8, i16, i32, i64, i128, isize, f32, f64);


pub trait NumBits: Numeric { const BITS: NumericBits; }

macro_rules! impl_bits {
    ($bits:ident in $($type:ty),+) => {
        $(
            impl NumBits for $type { const BITS: NumericBits = NumericBits::$bits; }
        )+
    };
}

impl_bits!(B8 in i8, u8);
impl_bits!(B16 in i16, u16);
impl_bits!(B32 in i32, u32, f32);
impl_bits!(B64 in i64, u64, f64);
impl_bits!(B128 in i128, u128);

impl_bits!(B32 in isize, usize);    //  For WebAssembly, at least


pub trait NumWholeness: Numeric {
    const INTEGRAL: bool;
}

pub trait Integral  :   Numeric
                    +   Ord + Eq
                    +   BitAnd + BitAndAssign + BitOr + BitOrAssign + BitXor + BitXorAssign
{}

macro_rules! impl_integral {
    ($($type:ty),+)    => {
        $(
            impl NumWholeness for $type { const INTEGRAL: bool = true; }
            impl Integral for $type {}
        )+
    };
}

impl_integral!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);


pub trait Floating: Signed {
    type ByteArray;
    type SameBitSize;

    //  std::f32/64::const:: constants
    const E                 : Self;
    const FRAC_1_PI         : Self;
    const FRAC_1_SQRT_2     : Self;
    const FRAC_2_PI         : Self;
    const FRAC_2_SQRT_PI    : Self;
    const FRAC_PI_2         : Self;
    const FRAC_PI_3         : Self;
    const FRAC_PI_4         : Self;
    const FRAC_PI_6         : Self;
    const FRAC_PI_8         : Self;
    const LN_2              : Self;
    const LN_10             : Self;
    const LOG2_10           : Self;
    const LOG2_E            : Self;
    const LOG10_2           : Self;
    const LOG10_E           : Self;
    const PI                : Self;
    const SQRT_2            : Self;
    const TAU               : Self;

    //  f32/64:: constants
    const RADIX             : u32;
    const MANTISSA_DIGITS   : u32;
    const DIGITS            : u32;
    const EPSILON           : Self;
    const MIN               : Self;
    const MIN_POSITIVE      : Self;
    const MAX               : Self;
    const MIN_EXP           : i32;
    const MAX_EXP           : i32;
    const MIN_10_EXP        : i32;
    const MAX_10_EXP        : i32;
    const NAN               : Self;
    const INFINITY          : Self;
    const NEG_INFINITY      : Self;

    fn acos(self) -> Self;
    fn acosh(self) -> Self;
    fn asin(self) -> Self;
    fn asinh(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn atanh(self) -> Self;
    fn cbrt(self) -> Self;
    fn ceil(self) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;
    fn classify(self) -> FpCategory;
    fn copysign(self, sign: Self) -> Self;
    fn cos(self) -> Self;
    fn cosh(self) -> Self;
    fn div_euclid(self, rhs: Self) -> Self;
    fn exp(self) -> Self;
    fn exp2(self) -> Self;
    fn exp_m1(self) -> Self;
    fn floor(self) -> Self;
    fn fract(self) -> Self;
    fn from_be_bytes(bytes: Self::ByteArray) -> Self;
    fn from_bits(bits: Self::SameBitSize) -> Self;
    fn from_le_bytes(bytes: Self::ByteArray) -> Self;
    fn from_ne_bytes(bytes: Self::ByteArray) -> Self;
    fn hypot(self, other: Self) -> Self;
    fn is_finite(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_nan(self) -> bool;
    fn is_normal(self) -> bool;
    fn is_sign_negative(self) -> bool;
    fn is_sign_positive(self) -> bool;
    fn is_subnormal(self) -> bool;
    fn ln(self) -> Self;
    fn ln_1p(self) -> Self;
    fn log(self, base: Self) -> Self;
    fn log10(self) -> Self;
    fn log2(self) -> Self;
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
    fn mul_add(self, a: Self, b: Self) -> Self;
    fn powf(self, n: Self) -> Self;
    fn powi(self, i: i32) -> Self;
    fn recip(self) -> Self;
    fn rem_euclid(self, other: Self) -> Self;
    fn round(self) -> Self;
    fn signum(self) -> Self;
    fn sin(self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn sinh(self) -> Self;
    fn sqrt(self) -> Self;
    fn tan(self) -> Self;
    fn tanh(self) -> Self;
    fn to_be_bytes(self) -> Self::ByteArray;
    fn to_bits(self) -> Self::SameBitSize;
    fn to_degrees(self) -> Self;
    fn to_le_bytes(self) -> Self::ByteArray;
    fn to_ne_bytes(self) -> Self::ByteArray;
    fn to_radians(self) -> Self;
    fn trunc(self) -> Self;

    fn is_close(self, to: Self, tolerance: Self) -> bool {
        (self - to).abs() <= tolerance
    }
}

macro_rules! float_constants {
    (@std_f_const $const:ident : $type:ident) => {
        const $const: $type = std::$type::consts::$const;
    };
    (@f_const $const:ident : $type:ident) => {
        const $const: $type = $type::$const;
    };
    (@f_const $const:ident : ( in $type:ident ) $const_type:ident) => {
        const $const: $const_type = $type::$const;
    };
    ($type:ident) => {
        float_constants!(@std_f_const   E               : $type);
        float_constants!(@std_f_const   FRAC_1_PI       : $type);
        float_constants!(@std_f_const   FRAC_1_SQRT_2   : $type);
        float_constants!(@std_f_const   FRAC_2_PI       : $type);
        float_constants!(@std_f_const   FRAC_2_SQRT_PI  : $type);
        float_constants!(@std_f_const   FRAC_PI_2       : $type);
        float_constants!(@std_f_const   FRAC_PI_3       : $type);
        float_constants!(@std_f_const   FRAC_PI_4       : $type);
        float_constants!(@std_f_const   FRAC_PI_6       : $type);
        float_constants!(@std_f_const   FRAC_PI_8       : $type);
        float_constants!(@std_f_const   LN_2            : $type);
        float_constants!(@std_f_const   LN_10           : $type);
        float_constants!(@std_f_const   LOG2_10         : $type);
        float_constants!(@std_f_const   LOG2_E          : $type);
        float_constants!(@std_f_const   LOG10_2         : $type);
        float_constants!(@std_f_const   LOG10_E         : $type);
        float_constants!(@std_f_const   PI              : $type);
        float_constants!(@std_f_const   SQRT_2          : $type);
        float_constants!(@std_f_const   TAU             : $type);

        float_constants!(@f_const       RADIX           : (in $type) u32);
        float_constants!(@f_const       MANTISSA_DIGITS : (in $type) u32);
        float_constants!(@f_const       DIGITS          : (in $type) u32);
        float_constants!(@f_const       EPSILON         : $type);
        float_constants!(@f_const       MIN             : $type);
        float_constants!(@f_const       MIN_POSITIVE    : $type);
        float_constants!(@f_const       MAX             : $type);
        float_constants!(@f_const       MIN_EXP         : (in $type) i32);
        float_constants!(@f_const       MAX_EXP         : (in $type) i32);
        float_constants!(@f_const       MIN_10_EXP      : (in $type) i32);
        float_constants!(@f_const       MAX_10_EXP      : (in $type) i32);
        float_constants!(@f_const       NAN             : $type);
        float_constants!(@f_const       INFINITY        : $type);
        float_constants!(@f_const       NEG_INFINITY    : $type);
    };
}

macro_rules! simple_float_methods {
    (@self_or) => { Self };
    (@self_or $type:tt) => { $type };
    (<$type:ty>::{$($name:ident $(-> $ret_type:tt)? ),+}) => {
        $( 
            #[inline(always)]
            fn $name(self) -> simple_float_methods!(@self_or $($ret_type)?) {
                <$type>::$name(self)
            }
        )+
    };
}
macro_rules! simple_predicates {
    (<$type:ty>::{$($name:ident),+}) => {
        $( 
            #[inline(always)]
            fn $name(self) -> bool {
                <$type>::$name(self)
            }
        )+
    };
}


macro_rules! to_other_type_methods {
    (<$base_type:ty => $result_type:path>::{$($name:ident),+}) => {
        $(
            #[inline(always)]
            fn $name(self) -> $result_type {
                <$base_type>::$name(self)
            }
        )+
    };
}


macro_rules! from_other_type_methods {
    (<$other_type:path => $base_type:ty>::{$($name:ident),+}) => {
        $(
            #[inline(always)]
            fn $name(other: $other_type) -> Self {
                <$base_type>::$name(other)
            }
        )+
    };
}

macro_rules! two_parameter_methods {
    (@self_or) => {
        Self
    };
    (@self_or $second_type:path) => {
        $second_type
    };
    (<$type:ty>::{$($name:ident $(2: $second_type:path)?),+}) => {
        $(
            #[inline(always)]
            fn $name(self, other: two_parameter_methods!{@self_or $($second_type)?}) -> Self {
                <$type>::$name(self, other)
            }
        )+
    };
}

macro_rules! three_parameter_methods {
    (<$type:ty>::{$($name:ident),+}) => {
        $(
            #[inline(always)]
            fn $name(self, a: Self, b: Self) -> Self {
                <$type>::$name(self, a, b)
            }
        )+
    }
}

macro_rules! impl_floating {
    ($type:ident, $bytes:literal, $same_bits:ty)    => {
        impl NumWholeness for $type {
            const INTEGRAL: bool = false;
        }

        impl Floating for $type {
            type ByteArray = [u8; $bytes];
            type SameBitSize = $same_bits;

            float_constants!{$type}

            simple_float_methods!{<$type>::{
                acos, acosh, asin, asinh, atan, atanh, classify -> FpCategory,
                cbrt, ceil, cos, cosh, exp, exp2, exp_m1, floor, fract, ln, ln_1p,
                log10, log2, recip, round, signum, sin, sin_cos -> (Self, Self),
                sinh, sqrt, tan, tanh, to_degrees, to_radians, trunc
            }}

            simple_predicates!{<$type>::{
                is_finite, is_infinite, is_nan, is_normal,
                is_sign_negative, is_sign_positive, is_subnormal
            }}

            to_other_type_methods!{<$type => Self::ByteArray>::{to_be_bytes, to_le_bytes, to_ne_bytes}}

            to_other_type_methods!{<$type => Self::SameBitSize>::{to_bits}}

            from_other_type_methods!{<Self::ByteArray => $type>::{from_be_bytes, from_le_bytes, from_ne_bytes}}

            from_other_type_methods!{<Self::SameBitSize => $type>::{from_bits}}

            two_parameter_methods!{<$type>::{
                atan2, copysign, div_euclid, hypot, log,
                max, min, powf, powi 2: i32, rem_euclid
            }}

            three_parameter_methods!{<$type>::{clamp, mul_add}}
        }
    };
}

impl_floating!{f32, 4, u32}
impl_floating!{f64, 8, u64}


// /// Infallible conversions that internally use
// /// Rust's `as` type casting. `AsFrom` is as similar
// /// as possible to `From`.
// pub trait AsFrom<Source>
//     where Self: Numeric, Source: Numeric
// {
//     fn as_from(s: Source) -> Self;
// }
// 
// impl<S, T> AsFrom<S> for T
//     where T: From<S>
// {
//     #[inline(always)]
//     fn as_from(s: S) -> Self { T::from(s) }
// }
// 
// #[allow(dead_code)]
// macro_rules! impl_as_from {
//     ($s:ty, $t:ty) => {
//         impl AsFrom<$s> for $t {
//             #[inline(always)]
//             fn as_from(s: $s) -> Self {
//                 s as $t
//             }
//         }
//     }
// }
// 
// #[cfg(target_pointer_width = "64")]
// impl_as_from!{i32, usize}
// 
// #[cfg(target_pointer_width = "64")]
// impl_as_from!{u32, isize}
// 
// #[cfg(target_pointer_width = "64")]
// impl_as_from!{i64, isize}
// 
// #[cfg(target_pointer_width = "64")]
// impl_as_from!{u64, usize}
// 
// #[cfg(target_pointer_width = "16")]
// impl_as_from!{isize, i16}
// 
// #[cfg(target_pointer_width = "16")]
// impl_as_from!{isize, u32}
// 
// #[cfg(target_pointer_width = "16")]
// impl_as_from!{isize, f32}
// 
// #[cfg(target_pointer_width = "16")]
// impl_as_from!{usize, u16}
// 
// #[cfg(target_pointer_width = "16")]
// impl_as_from!{usize, i32}
// 
// #[cfg(target_pointer_width = "16")]
// impl_as_from!{usize, f32}
// 
// /// Infallible conversions that internally use
// /// Rust's `as` type casting. `AsFrom` is as similar
// /// as possible to `From`.
// pub trait AsInto<Target>
//     where Self: Numeric, Target: Numeric
// {
//     fn as_into(self) -> Target;
// }
// 
// /// `AsInto` is the reciprocal of `AsFrom`. Every instance
// /// of `AsInto` is automatically constructed from the
// /// corresponding `AsFrom` instance.
// impl<Source, Target> AsInto<Target> for Source
//     where   Target: Numeric + AsFrom<Source>,
//             Source: Numeric
// {
//     #[inline(always)]
//     fn as_into(self) -> Target {
//         Target::as_from(self)
//     }
// }
// 
// 
/// Error used by several `TryAsFrom` and `TryAsInto`
/// instances when conversions fail due to loss of
/// precision.
#[derive(Clone, Copy, Debug)]
pub struct InsufficientPrecisionError<N>(N);

/// Possibly failing conversions that internally use
/// Rust's `as` type casting. `TryAsFrom` is as similar
/// as possible to `TryFrom`.
pub trait TryAsFrom<Source>
    where Self: Sized + Numeric, Self: Numeric
{
    fn try_as_from(s: Source) -> Result<Self, InsufficientPrecisionError<Source>>;
}

//impl<S: Numeric, T: Numeric + TryFrom<S>> TryAsFrom<S> for T {
//    fn try_as_from(s: S) -> Result<T, InsufficientPrecisionError<S>> {
//        T::try_from(s).map_err(|_| InsufficientPrecisionError(s))
//    }
//}

// impl<S: Numeric, T: AsFrom<S> + Numeric> TryAsFrom<S> for T {
//     #[inline(always)]
//     fn try_as_from(s: S) -> Result<Self, InsufficientPrecisionError<S>> {
//         Ok(T::as_from(s))
//     }
// }
// 
// impl<S: Numeric, T: TryFrom<S> + Numeric> TryAsFrom<S> for T {
//     fn try_as_from(s: S) -> Result<Self, InsufficientPrecisionError<S>> {
//         T::try_from(s).map_err(|_| InsufficientPrecisionError(s))
//     }
// }

macro_rules! impl_try_as_from {
    ($from:ty, $to:ty) => {
        impl TryAsFrom<$from> for $to {
            fn try_as_from(i: $from) -> Result<Self, InsufficientPrecisionError<$from>> {
                let f = i as $to;
                if f as $from == i {
                    Ok(f)
                }
                else {
                    Err(InsufficientPrecisionError(i))
                }
            }
        }
    };
}

//macro_rules! impl_try_as_from_bidir {
//    ($a:ty, $b:ty) => {
//        impl_try_as_from!{$a, $b}
//        impl_try_as_from!{$b, $a}
//    }
//}

impl_try_as_from!{f32, i16}
impl_try_as_from!{i16, f32}
impl_try_as_from!{u32, f32}
impl_try_as_from!{i32, f32}
impl_try_as_from!{f64, f32}
 
// impl_try_as_from_bidir!{i8, u8}
// impl_try_as_from_bidir!{i16, u16}
// impl_try_as_from_bidir!{i32, u32}
// impl_try_as_from_bidir!{i64, u64}
// impl_try_as_from_bidir!{i128, u128}
// impl_try_as_from_bidir!{isize, usize}
// 
// impl_try_as_from!{i16, i8}
// impl_try_as_from!{u16, i8}
// impl_try_as_from!{i32, i8}
// impl_try_as_from!{u32, i8}
// impl_try_as_from!{i64, i8}
// impl_try_as_from!{u64, i8}
// impl_try_as_from!{i128, i8}
// impl_try_as_from!{u128, i8}
// impl_try_as_from!{usize, i8}
// impl_try_as_from!{isize, i8}
// impl_try_as_from!{f32, i8}
// impl_try_as_from!{f64, i8}
// 
// impl_try_as_from!{i16, u8}
// impl_try_as_from!{u16, u8}
// impl_try_as_from!{i32, u8}
// impl_try_as_from!{u32, u8}
// impl_try_as_from!{i64, u8}
// impl_try_as_from!{u64, u8}
// impl_try_as_from!{i128, u8}
// impl_try_as_from!{u128, u8}
// impl_try_as_from!{usize, u8}
// impl_try_as_from!{isize, u8}
// impl_try_as_from!{f32, u8}
// impl_try_as_from!{f64, u8}
// 
// impl_try_as_from!{i32, i16}
// impl_try_as_from!{u32, i16}
// impl_try_as_from!{i64, i16}
// impl_try_as_from!{u64, i16}
// impl_try_as_from!{i128, i16}
// impl_try_as_from!{u128, i16}
// impl_try_as_from!{usize, i16}
// #[cfg(not(target_pointer_width = "16"))]
// impl_try_as_from!{isize, i16}
// impl_try_as_from!{f32, i16}
// impl_try_as_from!{f64, i16}
// 
// impl_try_as_from!{i32, u16}
// impl_try_as_from!{u32, u16}
// impl_try_as_from!{i64, u16}
// impl_try_as_from!{u64, u16}
// impl_try_as_from!{i128, u16}
// impl_try_as_from!{u128, u16}
// #[cfg(not(target_pointer_width = "16"))]
// impl_try_as_from!{usize, u16}
// impl_try_as_from!{isize, u16}
// impl_try_as_from!{f32, u16}
// impl_try_as_from!{f64, u16}
// 
// impl_try_as_from!{i64, i32}
// impl_try_as_from!{u64, i32}
// impl_try_as_from!{i128, i32}
// impl_try_as_from!{u128, i32}
// #[cfg(not(target_pointer_width = "16"))]
// impl_try_as_from!{usize, i32}
// #[cfg(target_pointer_width = "64")]
// impl_try_as_from!{isize, i32}
// impl_try_as_from!{f32, i32}
// impl_try_as_from!{f64, i32}
// 
// impl_try_as_from!{i64, u32}
// impl_try_as_from!{u64, u32}
// impl_try_as_from!{i128, u32}
// impl_try_as_from!{u128, u32}
// #[cfg(target_pointer_width = "64")]
// impl_try_as_from!{usize, u32}
// #[cfg(not(target_pointer_width = "16"))]
// impl_try_as_from!{isize, u32}
// impl_try_as_from!{f32, u32}
// impl_try_as_from!{f64, u32}
// 
// impl_try_as_from!{i128, i64}
// impl_try_as_from!{u128, i64}
// #[cfg(target_pointer_width= "64")]
// impl_try_as_from!{usize, i64}
// impl_try_as_from!{f32, i64}
// impl_try_as_from!{f64, i64}
// 
// impl_try_as_from!{i128, u64}
// impl_try_as_from!{u128, u64}
// #[cfg(target_pointer_width= "64")]
// impl_try_as_from!{isize, u64}
// impl_try_as_from!{f32, u64}
// impl_try_as_from!{f64, u64}
// 
// #[cfg(target_pointer_width = "16")]
// impl_try_as_from!{u16, isize}
// #[cfg(not(target_pointer_width = "64"))]
// impl_try_as_from!{u32, isize}
// #[cfg(target_pointer_width = "16")]
// impl_try_as_from!{i32, isize}
// #[cfg(not(target_pointer_width = "64"))]
// impl_try_as_from!{i64, isize}
// impl_try_as_from!{u64, isize}
// impl_try_as_from!{u128, isize}
// impl_try_as_from!{i128, isize}
// impl_try_as_from!{f32, isize}
// impl_try_as_from!{f64, isize}
// 
// #[cfg(target_pointer_width = "16")]
// impl_try_as_from!{i16, usize}
// #[cfg(not(target_pointer_width = "64"))]
// impl_try_as_from!{i32, usize}
// #[cfg(target_pointer_width = "16")]
// impl_try_as_from!{u32, usize}
// impl_try_as_from!{i64, usize}
// #[cfg(not(target_pointer_width = "64"))]
// impl_try_as_from!{u64, usize}
// impl_try_as_from!{i128, usize}
// impl_try_as_from!{u128, usize}
// #[cfg(target_pointer_width = "16")]
// impl_try_as_from!{f32, usize}
// impl_try_as_from!{f64, usize}
// 
// impl_try_as_from!{f64, f32}
// 
/// Possibly failing conversions that internally use
/// Rust's `as` type casting. `TryAsInto` is as similar
/// as possible to `TryInto`.
pub trait TryAsInto<Target>
    where Self: Numeric, Target: Numeric
{
    fn try_as_into(self) -> Result<Target, InsufficientPrecisionError<Self>>;
}

impl<Src, Tgt> TryAsInto<Tgt> for Src
    where Tgt: TryAsFrom<Src> + Numeric,
          Src: Numeric
{
    #[inline(always)]
    fn try_as_into(self) -> Result<Tgt, InsufficientPrecisionError<Self>> {
        <Tgt as TryAsFrom<Self>>::try_as_from(self)
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn closeness() {
        assert!(1.0f64.is_close(1.001, 0.0011));
        assert!(!1.0f64.is_close(1.001, 0.0005));
    }
}
