#![allow(dead_code)]

#[cfg(target_arch = "aarch64")]
use crate::engines::neon::Neon;
use crate::engines::scalar::Scalar;
#[cfg(target_arch = "wasm32")]
use crate::engines::wasm32::Wasm;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use crate::engines::{avx2::Avx2, sse2::Sse2, sse41::Sse41};

use crate::libm_ext::FloatExt;
use core::marker::PhantomData;

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;
#[cfg(target_arch = "wasm32")]
use core::arch::wasm32::*;
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

mod i8;

mod i16;

mod i32;

mod i64;

mod f32;

mod f64;

mod bit;

mod casts;

#[allow(non_camel_case_types)]
pub struct binary;

pub struct Ops<T, T2>(PhantomData<(T, T2)>);

macro_rules! with_feature_flag {
    (Avx2, $($r:tt)+) => {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        #[target_feature(enable = "avx2")]
        $($r)+
    };
    (Sse2, $($r:tt)+) => {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        #[target_feature(enable = "sse2")]
        $($r)+
    };
    (Sse41, $($r:tt)+) => {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        #[target_feature(enable = "sse4.1")]
        $($r)+
    };
    (Neon, $($r:tt)+) => {
        #[cfg(target_arch = "aarch64")]
        #[target_feature(enable = "neon")]
        $($r)+
    };
    (Wasm, $($r:tt)+) => {
        #[cfg(target_arch = "wasm32")]
        #[target_feature(enable = "simd128")]
        $($r)+
    };
    (Scalar, $($r:tt)+) => {
        $($r)+
    };
}
use with_feature_flag;

macro_rules! with_cfg_flag {
    (Avx2, $($r:tt)+) => {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        $($r)+
    };
    (Sse2, $($r:tt)+) => {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        $($r)+
    };
    (Sse41, $($r:tt)+) => {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        $($r)+
    };
    (Neon, $($r:tt)+) => {
        #[cfg(target_arch = "aarch64")]
        $($r)+
    };
    (Wasm, $($r:tt)+) => {
        #[cfg(target_arch = "wasm32")]
        $($r)+
    };
    (Scalar, $($r:tt)+) => {
        $($r)+
    };
}
use with_cfg_flag;

macro_rules! impl_op {
    (fn $name:ident < $scalar:ty > {
        $(
            for $engine:ident ($( $arg:ident : $arg_ty:ty ),*) $( -> $ret_ty:ty )? {
                $( $body:tt )*
            }
        )*
    }) => {
        $(
            with_cfg_flag!(
                $engine,
                impl Ops<$engine, $scalar> {
                    with_feature_flag!(
                        $engine,
                        #[inline]
                        pub unsafe fn $name($($arg: $arg_ty),*) $( -> $ret_ty )? {
                            $( $body )*
                        }
                    );
                }
            );
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
            with_cfg_flag!(
                $engine,
                impl Ops<$engine, $scalar> {
                    with_feature_flag!(
                        $engine,
                        #[inline]
                        pub unsafe fn $name < const $imm8: $imm8ty > ($($arg: $arg_ty),*) $( -> $ret_ty )? {
                            $( $body )*
                        }
                    );
                }
            );
        )*
    }
}
use impl_imm8_op;
