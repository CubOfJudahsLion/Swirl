//! Module  :   Int and Float
//! Purpose :   A library to establish numeric types and differentiate
//!             between integral and floating-point types.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue Jul 20 01:06:45 PM CST 2021
//! Modified:   Wed 04/20/2022 02:05 PM


use std::convert::{TryFrom, Infallible};
use std::num::FpCategory;
use std::ops::{
    Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign
};
use paste::paste;


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


/// Infallible numeric-type-to-numeric-type conversions,
/// with it and its mirror trait `NumInto` imitating
/// the functionality of `From` and `Into`.
pub trait NumFrom<Source>
    where Self: Sized
{
    fn num_from(s: Source) -> Self;
}

impl<Source> NumFrom<Source> for Source {
    #[inline(always)]
    fn num_from(s: Source) -> Self { s }
}

/// Mirror trait of `NumFrom`. Represents every numeric
/// type that can convert into another safely.
pub trait NumInto<Target>
{
    fn num_into(self) -> Target;
}

impl<S, T> NumInto<T> for S
    where T: NumFrom<S>
{
    #[inline(always)]
    fn num_into(self) -> T {
        S::num_from(self)
    }
}


/// Fallible numeric conversions. `TryNumFrom` is as similar
/// as possible to `TryFrom`.
pub trait TryNumFrom<Source>
    where Self: Sized
{
    type Error;
    fn try_num_from(t: Source) -> Result<Self, Self::Error>;
}

impl<T> TryNumFrom<T> for T {
    type Error = Infallible;

    #[inline(always)]
    fn try_num_from(t: T) -> Result<Self, Self::Error> {
        Ok(t)
    }
}


/// Error used by several `TryNumFrom` and `TryAsInto`
/// instances when conversions fail due to loss of
/// precision.
pub struct InsufficientPrecisionError;


//  A macro for implementing conversions that cannot fail...
macro_rules! impl_num_from_infallible {
    ($from:ty => $to:ty) => {
        impl NumFrom<$from> for $to {
            #[inline(always)]
            fn num_from(t: $from) -> Self {
                t as $to
            }
        }

        impl TryNumFrom<$from> for $to {
            type Error = Infallible;

            #[inline(always)]
            fn try_num_from(t: $from) -> Result<Self, Self::Error> {
                Ok(t as $to)
            }
        }
    };
}

//  ...and one for those that can.
macro_rules! impl_num_from_fallible {
    ($from:ty => $to:ty) => {
        impl TryNumFrom<$from> for $to {
            type Error = InsufficientPrecisionError;

            fn try_num_from(from: $from) -> Result<$to, Self::Error> {
                let to = from as $to;
                if to as $from == from {
                    Ok(to)
                }
                else {
                    Err(InsufficientPrecisionError)
                }
            }
        }
    };
}

#[cfg(target_pointer_width = "32")]
macro_rules! ptr_size {
    (size)          => { 32 };
    ($n:literal)    => { $n };
}
#[cfg(target_pointer_width = "64")]
macro_rules! ptr_size {
    (size)          => { 64 };
    ($n:literal)    => { $n };
}

//  Finally, one that will generate all of the instances.
macro_rules! impl_num_from_types {
    (@cmp   8          8) => { "eq" };
    (@cmp   8 $n:literal) => { "lt" };
    (@cmp  16          8) => { "gt" };
    (@cmp  16         16) => { "eq" };
    (@cmp  16 $n:literal) => { "lt" };
    (@cmp  32          8) => { "gt" };
    (@cmp  32         16) => { "gt" };
    (@cmp  32         32) => { "eq" };
    (@cmp  32 $n:literal) => { "lt" };
    (@cmp  64         64) => { "eq" };
    (@cmp  64        128) => { "lt" };
    (@cmp  64 $n:literal) => { "gt" };
    (@cmp 128        128) => { "eq" };
    (@cmp 128 $n:literal) => { "gt" };
    (@fallible $pre1:ident $size1:ty => $pre2:ident $size2:ty) => {
        paste!{
            impl_num_fallible!{[<$pre1 $size1>] => [<$pre2 $size2>]}
        }
    };
    (@infallible $pre1:ident $size1:literal => $pre2:ident $size2:literal) => {
        paste!{
            impl_num_infallible!{[<$pre1 $size1>] => [<$pre2 $size2>]}
        }
    };
    (@lt "lt" $pre1:ident $size1:tt $pre2:ident $size2:tt) => {
        impl_num_from_types!{@infallible $pre1 $size => $pre2 $size2}
    };
    (@lt $_:literal $pre1:ident $size1:tt $pre2:ident $size2:tt) => {
        impl_num_from_fallible!{@infallible $pre1 $size => $pre2 $size2}
    };
    (@between u $size1:tt => $pre2:ident $size2:tt) => {
        impl_num_from_types!{@lt impl_num_from_types!{@cmp ptr_size!($size1) ptr_size!($size2)} u $size1 => $pre2 $size2}
    };
    (@between i $size1:tt => u $size2:tt) => {
        impl_num_from_types!{@fallible i $size1 => u $size2}
    };
    (@between i $size1:tt => $pre2:ident $size2:tt) => {
        impl_num_from_types!{@lt impl_num_from_types!{@cmp ptr_size!($size1) ptr_size!($size2)} i $size1 => $pre2 $size2}
    };
    (@between f $size1:tt => f $size2:tt) => {
        impl_num_from_types!{@lt impl_num_from_types!{@cmp ptr_size!($size1) ptr_size!($size2)} i $size1 => $pre2 $size2}
    };
    (@between f $size1:tt => $pre2:ident $size2:tt) => {
        impl_num_from_types!{@fallible f $size1 => $pre2 $size2}
    };
    (@cmp_pre i i)                       => { "eq" };
    (@cmp_pre u u)                       => { "eq" };
    (@cmp_pre f f)                       => { "eq" };
    (@cmp_pre $pre1:ident $pre2:ident)   => { "ne" };
    (@ne "eq" "eq" $pre1:ident $size1:tt => $pre2:ident $size2:tt) => { };
    (@ne $cmp1:literal $cmp2:literal $pre1:ident $size1:tt => $pre2:ident $size2:tt) => {
        impl_num_from_types!{@between $pre1 $size1 => $pre2 $size2}
    };
    (@every $pre1:ident $size1:tt => ( $($pre2:ident $size2:tt),+ ) ) => {
        $(
            impl_num_from_types!{@ne
                impl_num_from_types!{@cmp_pre $pre1 $pre2}
                impl_num_from_types!(@cmp ptr_size!($size1) ptr_size!($size2))
                $pre1 $size1 => $pre2 $size2
            }
        )+
    };
    (@each_to_all $($pre1:ident $size1:tt),+ => $list:tt ) => {
        $(
            impl_num_from_types!{@every $pre1 $size1 => $list }
        )+
    };
    (@all_to_all $($pre:ident $size:tt),+) => {
        impl_num_from_types!{@each_to_all $($pre $size),+ => ( $($pre $size),+ ) }
    };
    () => {
        impl_num_from_types!{@all_to_all
            u 8, u 16, u 32, u 64, u 128, u size,
            i 8, i 16, i 32, i 64, i 128, i size,
            f 32, f 64
        }
    }
}

impl_num_from_types!{}


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
