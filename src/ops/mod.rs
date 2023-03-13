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

mod f32;
pub use self::f32::*;

mod f64;
pub use self::f64::*;

mod bit;
pub use self::bit::*;

mod casts;
pub use self::casts::*;

pub struct Ops<T>(PhantomData<T>);

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
    (fn $name:ident {
        $(
            for $engine:ident ($( $arg:ident : $arg_ty:ty ),*) $( -> $ret_ty:ty )? {
                $( $body:tt )*
            }
        )*
    }) => {
        $(
            impl Ops<$engine> {
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
