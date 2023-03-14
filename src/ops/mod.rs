#![allow(dead_code)]

use crate::engines::{avx2::Avx2, scalar::Scalar, sse2::Sse2, sse41::Sse41};
use crate::libm_ext::FloatExt;
use core::marker::PhantomData;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

mod i32;
pub use self::i32::*;

mod i64;
pub use self::i64::*;

mod f32;
pub use self::f32::*;

mod f64;
pub use self::f64::*;

mod bit;
pub use self::bit::*;

mod casts;
pub use self::casts::*;

#[allow(non_camel_case_types)]
pub struct binary;

pub struct Ops<T, T2>(PhantomData<(T, T2)>);

macro_rules! with_feature_flag {
    (Avx2, $($r:tt)+) => {
        #[target_feature(enable = "avx2")]
        $($r)+
    };
    (Sse2, $($r:tt)+) => {
        #[target_feature(enable = "sse2")]
        $($r)+
    };
    (Sse41, $($r:tt)+) => {
        #[target_feature(enable = "sse4.1")]
        $($r)+
    };
    (Scalar, $($r:tt)+) => {
        $($r)+
    };
}
use with_feature_flag;

macro_rules! impl_op {
    (fn $name:ident < $scalar:ty > {
        $(
            for $engine:ident ($( $arg:ident : $arg_ty:ty ),*) $( -> $ret_ty:ty )? {
                $( $body:tt )*
            }
        )*
    }) => {
        $(
            impl Ops<$engine, $scalar> {
                with_feature_flag!(
                    $engine,
                    #[inline]
                    pub unsafe fn $name($($arg: $arg_ty),*) $( -> $ret_ty )? {
                        $( $body )*
                    }
                );
            }
        )*
    }
}
use impl_op;

macro_rules! impl_imm8_op {
    (fn $name:ident < $scalar:ty, const $imm8:ident: $imm8ty:ty > {
        $(
            for $engine:ident ($( $arg:ident : $arg_ty:ty ),*) $( -> $ret_ty:ty )? {
                $( $body:tt )*
            }
        )*
    }) => {
        $(
            impl Ops<$engine, $scalar> {
                with_feature_flag!(
                    $engine,
                    #[inline]
                    pub unsafe fn $name < const $imm8: $imm8ty > ($($arg: $arg_ty),*) $( -> $ret_ty )? {
                        $( $body )*
                    }
                );
            }
        )*
    }
}
use impl_imm8_op;
